//! Deterministic family lock generation from locally verified signed tags.

mod git;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use self::git::{digest_generated_client, digest_tagged_tree, tag_commit, verify_tag};
use crate::coord::CoordError;

const LOCK_FILE: &str = "family.lock";
const ALLOWED_SIGNERS: &str = "release/allowed_signers";
const SCHEMA_PREFIX: &str = "crates/bullet-wire";

#[derive(Deserialize)]
struct Manifest {
    family: String,
    required_repos: Vec<String>,
    repo: Vec<ManifestRepo>,
}

#[derive(Deserialize)]
struct ManifestRepo {
    name: String,
    path: PathBuf,
}

#[derive(Serialize)]
struct FamilyLock {
    schema_version: &'static str,
    family: String,
    tag: String,
    schema_bundle_hash: String,
    member: Vec<LockedMember>,
}

#[derive(Serialize)]
struct LockedMember {
    name: String,
    tag: String,
    commit_oid: String,
    schema_bundle_hash: String,
    release_signing_identity: String,
    generated_client_hash: String,
}

pub fn run(root: &Path, args: &[String]) -> Result<String, CoordError> {
    let (action, tag) = parse_args(args)?;
    let bytes = render(root, &tag)?;
    let path = root.join(LOCK_FILE);
    match action.as_str() {
        "generate" => {
            atomic_write(&path, &bytes)?;
            Ok(format!("generated {} for {tag}", path.display()))
        }
        "check" => {
            let current = fs::read(&path).map_err(CoordError::io)?;
            if current != bytes {
                return Err(CoordError::new(
                    "FAMILY_LOCK_DRIFT",
                    format!("{} differs from verified local tags", path.display()),
                ));
            }
            Ok(format!("{} matches {tag}", path.display()))
        }
        _ => Err(CoordError::new("USAGE", lock_usage())),
    }
}

fn parse_args(args: &[String]) -> Result<(String, String), CoordError> {
    if args
        .first()
        .is_none_or(|action| !matches!(action.as_str(), "generate" | "check"))
    {
        return Err(CoordError::new("USAGE", lock_usage()));
    }
    if args.len() != 3 || args[1] != "--tag" {
        return Err(CoordError::new("USAGE", lock_usage()));
    }
    let tag = &args[2];
    if tag.len() > 128
        || !tag.starts_with('v')
        || tag
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-')))
    {
        return Err(CoordError::new(
            "INVALID_RELEASE_TAG",
            "release tag must be a bounded v-prefixed ASCII version",
        ));
    }
    Ok((args[0].clone(), tag.clone()))
}

fn lock_usage() -> &'static str {
    "usage: bullet-family [--root PATH] lock <generate|check> --tag <version>"
}

fn render(root: &Path, tag: &str) -> Result<Vec<u8>, CoordError> {
    let manifest_text =
        fs::read_to_string(root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let manifest: Manifest = toml::from_str(&manifest_text)
        .map_err(|error| CoordError::new("INVALID_FAMILY_MANIFEST", error.to_string()))?;
    let repos = indexed_repos(root, &manifest)?;
    let farm = repos.get("bullet-farm").ok_or_else(|| {
        CoordError::new(
            "FAMILY_MEMBER_MISSING",
            "manifest has no bullet-farm member",
        )
    })?;
    let schema_bundle_hash = digest_tagged_tree(farm, tag, SCHEMA_PREFIX)?;
    let allowed_signers = root.join(ALLOWED_SIGNERS);
    if !allowed_signers.is_file() {
        return Err(CoordError::new(
            "ALLOWED_SIGNERS_MISSING",
            format!("{} does not exist", allowed_signers.display()),
        ));
    }
    let mut members = Vec::with_capacity(manifest.required_repos.len());
    for name in &manifest.required_repos {
        let repo = repos.get(name).ok_or_else(|| {
            CoordError::new(
                "FAMILY_MEMBER_MISSING",
                format!("manifest has no {name} member"),
            )
        })?;
        let commit_oid = tag_commit(repo, tag)?;
        let release_signing_identity = verify_tag(repo, tag, &allowed_signers)?;
        let generated_client_hash = digest_generated_client(repo, tag, name)?;
        members.push(LockedMember {
            name: name.clone(),
            tag: tag.to_owned(),
            commit_oid,
            schema_bundle_hash: schema_bundle_hash.clone(),
            release_signing_identity,
            generated_client_hash,
        });
    }
    let lock = FamilyLock {
        schema_version: "2",
        family: manifest.family,
        tag: tag.to_owned(),
        schema_bundle_hash,
        member: members,
    };
    let text = toml::to_string_pretty(&lock)
        .map_err(|error| CoordError::new("FAMILY_LOCK_ENCODE_FAILED", error.to_string()))?;
    Ok(text.into_bytes())
}

fn indexed_repos(
    root: &Path,
    manifest: &Manifest,
) -> Result<BTreeMap<String, PathBuf>, CoordError> {
    let required: BTreeSet<_> = manifest.required_repos.iter().cloned().collect();
    if required.len() != manifest.required_repos.len() {
        return Err(CoordError::new(
            "DUPLICATE_FAMILY_MEMBER",
            "required_repos contains duplicates",
        ));
    }
    let mut repos = BTreeMap::new();
    for entry in &manifest.repo {
        if entry.path.file_name().and_then(|name| name.to_str()) != Some(entry.name.as_str()) {
            return Err(CoordError::new(
                "INVALID_MEMBER_PATH",
                format!("manifest path does not end with {}", entry.name),
            ));
        }
        if repos
            .insert(entry.name.clone(), root.join(&entry.name))
            .is_some()
        {
            return Err(CoordError::new(
                "DUPLICATE_FAMILY_MEMBER",
                format!("manifest repeats {}", entry.name),
            ));
        }
    }
    if repos.keys().cloned().collect::<BTreeSet<_>>() != required {
        return Err(CoordError::new(
            "FAMILY_MEMBER_SET_MISMATCH",
            "repo entries must exactly match required_repos",
        ));
    }
    Ok(repos)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), CoordError> {
    let parent = path
        .parent()
        .ok_or_else(|| CoordError::new("INVALID_LOCK_PATH", "family lock has no parent"))?;
    let temporary = parent.join(format!(".family.lock.{}.tmp", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)
        .map_err(CoordError::io)?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        let _ = fs::remove_file(&temporary);
        return Err(CoordError::io(error));
    }
    fs::rename(&temporary, path).map_err(CoordError::io)?;
    let directory = fs::File::open(parent).map_err(CoordError::io)?;
    directory.sync_all().map_err(CoordError::io)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_tag_parser_is_fail_closed() {
        assert!(parse_args(&["generate".into(), "--tag".into(), "v0.1.0-alpha.1".into()]).is_ok());
        for tag in ["alpha", "v1/escape", "v1..\n"] {
            assert!(parse_args(&["generate".into(), "--tag".into(), tag.into()]).is_err());
        }
    }

    #[test]
    fn framed_digest_distinguishes_field_boundaries() {
        let digest = |parts: &[&[u8]]| {
            let mut hasher = blake3::Hasher::new();
            for part in parts {
                git::frame(&mut hasher, part);
            }
            hasher.finalize()
        };
        assert_ne!(digest(&[b"ab", b"c"]), digest(&[b"a", b"bc"]));
    }

    #[test]
    fn ssh_status_requires_one_ed25519_identity() {
        let good = "Good \"git\" signature for bot@jekko.ai with ED25519 key SHA256:abc+123\n";
        assert_eq!(
            git::signer_identity(good, "v1").unwrap(),
            "bot@jekko.ai|ed25519|SHA256:abc+123"
        );
        for denied in [
            "",
            "Good \"file\" signature for bot@jekko.ai with ED25519 key SHA256:abc\n",
            "Good \"git\" signature for bot@jekko.ai with RSA key SHA256:abc\n",
            "Good \"git\" signature for bad principal with ED25519 key SHA256:abc\n",
        ] {
            assert!(git::signer_identity(denied, "v1").is_err());
        }
    }
}
