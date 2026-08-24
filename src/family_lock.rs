//! Deterministic family lock generation from locally verified signed tags.

mod git;
mod schema;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub use self::schema::{FamilyLock, LOCK_SCHEMA_VERSION, LockedFile, LockedMember, load, parse};
use self::{
    git::{
        digest_dependency_lockfiles, digest_generated_artifacts, digest_tagged_tree, tag_commit,
        tag_tree, verify_tag,
    },
    schema::{validate, validate_jeryu_source, validate_tag},
};
use crate::coord::CoordError;

const LOCK_FILE: &str = "family.lock";
const ALLOWED_SIGNERS: &str = "release/allowed_signers";
const SCHEMA_PREFIX: &str = "crates/bullet-wire";

#[derive(Deserialize)]
struct Manifest {
    family: String,
    required_repos: Vec<String>,
    #[serde(default)]
    repo: Vec<ManifestRepo>,
}

#[derive(Deserialize)]
struct ManifestRepo {
    name: String,
    path: PathBuf,
    #[serde(default)]
    jeryu_url: Option<String>,
    #[serde(default)]
    jeryu_slug: Option<String>,
}

pub fn run(root: &Path, args: &[String]) -> Result<String, CoordError> {
    let (action, tag) = parse_args(args)?;
    let family_root = resolve_family_root(root)?;
    let path = family_root.join("bullet-farm").join(LOCK_FILE);
    match action.as_str() {
        "generate" => {
            let bytes = render(&family_root, &tag, "HEAD")?;
            atomic_write(&path, &bytes)?;
            Ok(format!("generated {} for {tag}", path.display()))
        }
        "check" | "verify" => {
            let current = fs::read(&path).map_err(CoordError::io)?;
            let lock = parse(&current)?;
            if lock.tag != tag {
                return Err(CoordError::new(
                    "FAMILY_LOCK_TAG_MISMATCH",
                    format!("family.lock binds {}, not requested {tag}", lock.tag),
                ));
            }
            let repos = verification_repos(&family_root, &lock)?;
            let farm = repos.get("bullet-farm").ok_or_else(|| {
                CoordError::new(
                    "FAMILY_MEMBER_MISSING",
                    "manifest has no bullet-farm member",
                )
            })?;
            let allowed_signers = farm.join(ALLOWED_SIGNERS);
            verify_hub_checkout(&lock, farm, &allowed_signers)?;
            for member in &lock.member {
                let repo = repos.get(&member.name).ok_or_else(|| {
                    CoordError::new(
                        "FAMILY_MEMBER_MISSING",
                        format!("manifest has no {} member", member.name),
                    )
                })?;
                verify_locked_checkout(member, repo, &allowed_signers)?;
            }
            Ok(format!("{} matches {tag}", path.display()))
        }
        _ => Err(CoordError::new("USAGE", lock_usage())),
    }
}

fn resolve_family_root(root: &Path) -> Result<PathBuf, CoordError> {
    if root.file_name().and_then(|name| name.to_str()) == Some("bullet-farm")
        && let Some(parent) = root
            .parent()
            .filter(|path| path.join("repos.manifest.toml").is_file())
    {
        return Ok(parent.to_path_buf());
    }
    if root.join("repos.manifest.toml").is_file() {
        return Ok(root.to_path_buf());
    }
    Err(CoordError::new(
        "FAMILY_MANIFEST_MISSING",
        format!(
            "{} is neither a split-family root nor its bullet-farm checkout",
            root.display()
        ),
    ))
}

fn parse_args(args: &[String]) -> Result<(String, String), CoordError> {
    if args
        .first()
        .is_none_or(|action| !matches!(action.as_str(), "generate" | "check" | "verify"))
    {
        return Err(CoordError::new("USAGE", lock_usage()));
    }
    if args.len() != 3 || args[1] != "--tag" {
        return Err(CoordError::new("USAGE", lock_usage()));
    }
    let tag = &args[2];
    validate_tag(tag).map_err(|_| CoordError::new("INVALID_RELEASE_TAG", "invalid release tag"))?;
    Ok((args[0].clone(), tag.clone()))
}

fn lock_usage() -> &'static str {
    "usage: bullet-family [--root PATH] lock <generate|check|verify> --tag <version>"
}

fn render(root: &Path, tag: &str, hub_revision: &str) -> Result<Vec<u8>, CoordError> {
    let manifest_text =
        fs::read_to_string(root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let manifest: Manifest = toml::from_str(&manifest_text)
        .map_err(|error| CoordError::new("INVALID_FAMILY_MANIFEST", error.to_string()))?;
    let repos = indexed_repos(root, &manifest)?;
    let sources = authenticated_sources(&manifest)?;
    let farm = repos.get("bullet-farm").ok_or_else(|| {
        CoordError::new(
            "FAMILY_MEMBER_MISSING",
            "manifest has no bullet-farm member",
        )
    })?;
    let schema_bundle_hash = digest_tagged_tree(farm, hub_revision, SCHEMA_PREFIX)?;
    let allowed_signers = farm.join(ALLOWED_SIGNERS);
    if !allowed_signers.is_file() {
        return Err(CoordError::new(
            "ALLOWED_SIGNERS_MISSING",
            format!("{} does not exist", allowed_signers.display()),
        ));
    }
    let mut members = Vec::with_capacity(manifest.required_repos.len().saturating_sub(1));
    for name in manifest
        .required_repos
        .iter()
        .filter(|name| name.as_str() != "bullet-farm")
    {
        let repo = repos.get(name).ok_or_else(|| {
            CoordError::new(
                "FAMILY_MEMBER_MISSING",
                format!("manifest has no {name} member"),
            )
        })?;
        let commit_oid = tag_commit(repo, tag)?;
        let tree_oid = tag_tree(repo, tag)?;
        let release_signing_identity = verify_tag(repo, tag, &allowed_signers)?;
        let lockfile = digest_dependency_lockfiles(repo, tag, name)?;
        let artifact = digest_generated_artifacts(repo, tag)?;
        let source = sources.get(name).cloned().flatten();
        members.push(LockedMember {
            name: name.clone(),
            jeryu_url: source.as_ref().map(|source| source.0.clone()),
            jeryu_slug: source.map(|source| source.1),
            tag: tag.to_owned(),
            commit_oid,
            tree_oid,
            release_signing_identity,
            lockfile,
            artifact,
        });
    }
    let lock = FamilyLock {
        schema_version: LOCK_SCHEMA_VERSION.to_owned(),
        family: manifest.family,
        tag: tag.to_owned(),
        schema_bundle_hash,
        member: members,
    };
    validate(&lock)?;
    let text = toml::to_string_pretty(&lock)
        .map_err(|error| CoordError::new("FAMILY_LOCK_ENCODE_FAILED", error.to_string()))?;
    Ok(text.into_bytes())
}

fn authenticated_sources(
    manifest: &Manifest,
) -> Result<BTreeMap<String, Option<(String, String)>>, CoordError> {
    let mut sources = BTreeMap::new();
    for repo in &manifest.repo {
        let source = match (&repo.jeryu_url, &repo.jeryu_slug) {
            (Some(url), Some(slug)) => {
                validate_jeryu_source(url, slug).map_err(|error| {
                    CoordError::new("INVALID_SOURCE_METADATA", format!("{}: {error}", repo.name))
                })?;
                if slug.rsplit('/').next() != Some(repo.name.as_str()) {
                    return Err(CoordError::new(
                        "INVALID_SOURCE_METADATA",
                        format!("{} Jeryu slug does not bind its member name", repo.name),
                    ));
                }
                Some((url.clone(), slug.clone()))
            }
            (None, None) if repo.name == "bullet-farm" => None,
            (None, _) => {
                return Err(CoordError::new(
                    "SOURCE_METADATA_UNAVAILABLE",
                    format!(
                        "{} lacks an authenticated jeryu_url; restore source authentication, publish signed tags, and update the manifest before generating a lock",
                        repo.name
                    ),
                ));
            }
            (Some(_), None) => {
                return Err(CoordError::new(
                    "INVALID_SOURCE_METADATA",
                    format!("{} must provide both jeryu_url and jeryu_slug", repo.name),
                ));
            }
        };
        sources.insert(repo.name.clone(), source);
    }
    Ok(sources)
}

pub fn verify_locked_checkout(
    member: &LockedMember,
    repo: &Path,
    allowed_signers: &Path,
) -> Result<(), CoordError> {
    git::verify_locked_checkout(member, repo, allowed_signers)
}

pub fn verify_hub_checkout(
    lock: &FamilyLock,
    repo: &Path,
    allowed_signers: &Path,
) -> Result<String, CoordError> {
    let tagged_commit = tag_commit(repo, &lock.tag)?;
    let tagged_tree = tag_tree(repo, &lock.tag)?;
    if git::head_commit(repo)? != tagged_commit || git::head_tree(repo)? != tagged_tree {
        return Err(CoordError::new(
            "HUB_TAG_SUBJECT_MISMATCH",
            "the signed hub tag does not resolve to the invoking hub HEAD/tree",
        ));
    }
    let schema_bundle_hash = digest_tagged_tree(repo, &lock.tag, SCHEMA_PREFIX)?;
    if schema_bundle_hash != lock.schema_bundle_hash {
        return Err(CoordError::new(
            "HUB_SCHEMA_MISMATCH",
            "the signed hub tag does not contain the locked schema bundle",
        ));
    }
    let tagged_lock = parse(&git::verified_blob(repo, &lock.tag, LOCK_FILE)?)?;
    if &tagged_lock != lock {
        return Err(CoordError::new(
            "HUB_LOCK_MISMATCH",
            "the invoking family.lock is not the exact lock in the signed hub tag",
        ));
    }
    verify_tag(repo, &lock.tag, allowed_signers)
}

fn verification_repos(
    root: &Path,
    lock: &FamilyLock,
) -> Result<BTreeMap<String, PathBuf>, CoordError> {
    let manifest_text =
        fs::read_to_string(root.join("repos.manifest.toml")).map_err(CoordError::io)?;
    let manifest: Manifest = toml::from_str(&manifest_text)
        .map_err(|error| CoordError::new("INVALID_FAMILY_MANIFEST", error.to_string()))?;
    if manifest.family != "bullet-farm" {
        return Err(CoordError::new(
            "INVALID_FAMILY_MANIFEST",
            "family manifest does not describe bullet-farm",
        ));
    }
    lock.validate_required_members(&manifest.required_repos)?;
    let mut repos = BTreeMap::new();
    repos.insert("bullet-farm".to_owned(), root.join("bullet-farm"));
    for member in &lock.member {
        repos.insert(member.name.clone(), root.join(&member.name));
    }
    Ok(repos)
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
        crate::coord::validate_repo_name(&entry.name)?;
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

    #[test]
    fn family_root_resolves_from_split_root_and_hub_checkout() {
        let outer =
            std::env::temp_dir().join(format!("bullet-family-lock-root-{}", std::process::id()));
        if outer.exists() {
            fs::remove_dir_all(&outer).unwrap();
        }
        let hub = outer.join("bullet-farm");
        fs::create_dir_all(&hub).unwrap();
        fs::write(
            outer.join("repos.manifest.toml"),
            "family = \"bullet-farm\"\n",
        )
        .unwrap();
        assert_eq!(resolve_family_root(&hub).unwrap(), outer);
        assert_eq!(resolve_family_root(&outer).unwrap(), outer);
        fs::remove_dir_all(outer).unwrap();
    }
}
