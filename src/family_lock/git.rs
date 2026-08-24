//! Signed-tag verification and tagged-tree hashing for family lock generation.

use std::path::Path;
use std::process::Command;

use crate::coord::CoordError;

const GIT_BIN: &str = "/usr/bin/git";

pub(super) fn tag_commit(repo: &Path, tag: &str) -> Result<String, CoordError> {
    let object_type = git(repo, &["cat-file", "-t", &format!("refs/tags/{tag}")])?;
    if object_type.trim() != "tag" {
        return Err(CoordError::new(
            "UNSIGNED_OR_LIGHTWEIGHT_TAG",
            format!("{tag} in {} is not an annotated tag", repo.display()),
        ));
    }
    let oid = git(
        repo,
        &["rev-parse", "--verify", &format!("{tag}^{{commit}}")],
    )?;
    let oid = oid.trim();
    if oid.len() != 40 || !oid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(CoordError::new(
            "INVALID_TAG_COMMIT",
            format!("{tag} resolved to an invalid commit OID"),
        ));
    }
    Ok(oid.to_owned())
}

pub(super) fn verify_tag(
    repo: &Path,
    tag: &str,
    allowed_signers: &Path,
) -> Result<String, CoordError> {
    let allowed = allowed_signers.to_str().ok_or_else(|| {
        CoordError::new("INVALID_SIGNER_PATH", "allowed-signers path is not UTF-8")
    })?;
    let output = git_output(
        repo,
        &[
            "-c",
            "gpg.format=ssh",
            "-c",
            &format!("gpg.ssh.allowedSignersFile={allowed}"),
            "verify-tag",
            "--raw",
            tag,
        ],
    )?;
    let status = String::from_utf8(output.stderr).map_err(|_| {
        CoordError::new(
            "INVALID_GIT_OUTPUT",
            "Git emitted non-UTF-8 signature status",
        )
    })?;
    signer_identity(&status, tag)
}

pub(super) fn signer_identity(status: &str, tag: &str) -> Result<String, CoordError> {
    let lines: Vec<_> = status
        .lines()
        .filter_map(|line| line.strip_prefix("Good \"git\" signature for "))
        .collect();
    if lines.len() != 1 {
        return Err(CoordError::new(
            "TAG_SIGNER_IDENTITY_MISSING",
            format!("{tag} has no single verified SSH signer status"),
        ));
    }
    let (principal, fingerprint) = lines[0].split_once(" with ED25519 key ").ok_or_else(|| {
        CoordError::new(
            "UNSUPPORTED_TAG_SIGNATURE",
            format!("{tag} is not signed by an Ed25519 SSH identity"),
        )
    })?;
    if principal.is_empty()
        || principal.bytes().any(|byte| byte.is_ascii_whitespace())
        || !fingerprint.starts_with("SHA256:")
        || fingerprint.bytes().any(|byte| {
            !(byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'+' | b'/' | b'='))
        })
    {
        return Err(CoordError::new(
            "INVALID_TAG_SIGNER_IDENTITY",
            format!("{tag} returned a malformed signer identity"),
        ));
    }
    Ok(format!("{principal}|ed25519|{fingerprint}"))
}

pub(super) fn digest_tagged_tree(
    repo: &Path,
    tag: &str,
    prefix: &str,
) -> Result<String, CoordError> {
    let listing = git(repo, &["ls-tree", "-r", "--name-only", tag, "--", prefix])?;
    let paths: Vec<_> = listing.lines().filter(|path| !path.is_empty()).collect();
    if paths.is_empty() {
        return Err(CoordError::new(
            "SCHEMA_BUNDLE_MISSING",
            format!("{tag} has no {prefix} tree"),
        ));
    }
    digest_tagged_files(repo, tag, "bullet.family.schema-bundle.v1", &paths)
}

pub(super) fn digest_generated_client(
    repo: &Path,
    tag: &str,
    name: &str,
) -> Result<String, CoordError> {
    let paths: &[&str] = match name {
        "bullet-kernel" => &["contracts/generated/api.ts"],
        "bullet-portal" => &["src/generated/api.ts"],
        "bullet-farm" | "bullet-git" => &[],
        _ => {
            return Err(CoordError::new(
                "UNSUPPORTED_FAMILY_MEMBER",
                format!("no generated-client policy for {name}"),
            ));
        }
    };
    digest_tagged_files(repo, tag, "bullet.family.generated-client.v1", paths)
}

fn digest_tagged_files(
    repo: &Path,
    tag: &str,
    domain: &str,
    paths: &[&str],
) -> Result<String, CoordError> {
    let mut hasher = blake3::Hasher::new();
    frame(&mut hasher, domain.as_bytes());
    for path in paths {
        frame(&mut hasher, path.as_bytes());
        let bytes = git_bytes(repo, &["show", &format!("{tag}:{path}")])?;
        frame(&mut hasher, &bytes);
    }
    Ok(format!("blake3:{}", hasher.finalize().to_hex()))
}

pub(super) fn frame(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&(bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

fn git(repo: &Path, args: &[&str]) -> Result<String, CoordError> {
    String::from_utf8(git_bytes(repo, args)?)
        .map_err(|_| CoordError::new("INVALID_GIT_OUTPUT", "Git emitted non-UTF-8 metadata"))
}

fn git_bytes(repo: &Path, args: &[&str]) -> Result<Vec<u8>, CoordError> {
    Ok(git_output(repo, args)?.stdout)
}

fn git_output(repo: &Path, args: &[&str]) -> Result<std::process::Output, CoordError> {
    let output = Command::new(GIT_BIN)
        .arg("-C")
        .arg(repo)
        .args(args)
        .env_clear()
        .env("LC_ALL", "C")
        .output()
        .map_err(CoordError::io)?;
    if !output.status.success() {
        return Err(CoordError::new(
            "GIT_VERIFICATION_FAILED",
            format!(
                "Git verification failed in {} for {:?}",
                repo.display(),
                args
            ),
        ));
    }
    Ok(output)
}
