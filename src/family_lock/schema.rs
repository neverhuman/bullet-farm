//! Strict, reusable schema for an installable Bullet Farm family lock.

use std::{collections::BTreeSet, fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::coord::CoordError;

pub const LOCK_SCHEMA_VERSION: &str = "3";
const MAX_LOCK_BYTES: u64 = 1024 * 1024;
const MAX_MEMBERS: usize = 64;
const MAX_FILES_PER_MEMBER: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyLock {
    pub schema_version: String,
    pub family: String,
    pub tag: String,
    pub schema_bundle_hash: String,
    pub member: Vec<LockedMember>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LockedMember {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jeryu_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jeryu_slug: Option<String>,
    pub tag: String,
    pub commit_oid: String,
    pub tree_oid: String,
    pub release_signing_identity: String,
    pub lockfile: Vec<LockedFile>,
    pub artifact: Vec<LockedFile>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LockedFile {
    pub path: String,
    pub digest: String,
}

impl FamilyLock {
    pub fn member(&self, name: &str) -> Option<&LockedMember> {
        self.member.iter().find(|member| member.name == name)
    }

    pub fn validate_required_members(&self, required: &[String]) -> Result<(), CoordError> {
        let all = required.iter().map(String::as_str).collect::<BTreeSet<_>>();
        if all.len() != required.len() {
            return Err(invalid("required family member names contain duplicates"));
        }
        if !all.contains("bullet-farm") {
            return Err(invalid("required family members omit bullet-farm"));
        }
        let expected = all
            .into_iter()
            .filter(|name| *name != "bullet-farm")
            .collect::<BTreeSet<_>>();
        let actual = self
            .member
            .iter()
            .map(|member| member.name.as_str())
            .collect::<BTreeSet<_>>();
        if actual != expected {
            return Err(invalid(
                "locked members do not exactly match the family manifest",
            ));
        }
        Ok(())
    }
}

pub fn load(path: &Path) -> Result<FamilyLock, CoordError> {
    let metadata = fs::symlink_metadata(path).map_err(CoordError::io)?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err(invalid("family.lock must be a regular, non-symlink file"));
    }
    if metadata.len() > MAX_LOCK_BYTES {
        return Err(invalid("family.lock exceeds the 1 MiB admission limit"));
    }
    parse(&fs::read(path).map_err(CoordError::io)?)
}

pub fn parse(bytes: &[u8]) -> Result<FamilyLock, CoordError> {
    if bytes.len() as u64 > MAX_LOCK_BYTES {
        return Err(invalid("family.lock exceeds the 1 MiB admission limit"));
    }
    let text =
        std::str::from_utf8(bytes).map_err(|_| invalid("family.lock must contain valid UTF-8"))?;
    if text
        .bytes()
        .any(|byte| byte == 0 || (byte < 0x20 && !matches!(byte, b'\n' | b'\r' | b'\t')))
    {
        return Err(invalid("family.lock contains a forbidden control byte"));
    }
    let table: toml::Table =
        toml::from_str(text).map_err(|error| invalid(format!("invalid TOML: {error}")))?;
    let version = table
        .get("schema_version")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| invalid("family.lock has no string schema_version"))?;
    if version != LOCK_SCHEMA_VERSION {
        return Err(CoordError::new(
            "UNSUPPORTED_SCHEMA",
            format!(
                "family.lock schema {version} is not installable; remove it or regenerate schema {LOCK_SCHEMA_VERSION} from authenticated signed tags"
            ),
        ));
    }
    let lock: FamilyLock =
        toml::from_str(text).map_err(|error| invalid(format!("invalid TOML: {error}")))?;
    validate(&lock)?;
    Ok(lock)
}

pub(crate) fn validate(lock: &FamilyLock) -> Result<(), CoordError> {
    if lock.schema_version != LOCK_SCHEMA_VERSION {
        return Err(CoordError::new(
            "UNSUPPORTED_SCHEMA",
            format!(
                "family.lock schema {} is not installable; regenerate schema {LOCK_SCHEMA_VERSION} from authenticated signed tags",
                lock.schema_version
            ),
        ));
    }
    if lock.family != "bullet-farm" {
        return Err(invalid("family must be bullet-farm"));
    }
    validate_tag(&lock.tag)?;
    validate_digest("schema_bundle_hash", &lock.schema_bundle_hash)?;
    if lock.member.is_empty() || lock.member.len() > MAX_MEMBERS {
        return Err(invalid("family.lock must contain 1..=64 members"));
    }
    let mut names = BTreeSet::new();
    for member in &lock.member {
        if member.name == "bullet-farm" {
            return Err(invalid(
                "bullet-farm is the signed top-level subject and must not be a member entry",
            ));
        }
        validate_member(member, &lock.tag)?;
        if !names.insert(member.name.as_str()) {
            return Err(invalid(format!(
                "family.lock repeats member {}",
                member.name
            )));
        }
    }
    Ok(())
}

fn validate_member(member: &LockedMember, family_tag: &str) -> Result<(), CoordError> {
    validate_atom("member name", &member.name, 64)?;
    if member.tag != family_tag {
        return Err(invalid(format!(
            "{} tag does not match the family tag",
            member.name
        )));
    }
    validate_git_oid("commit_oid", &member.commit_oid)?;
    validate_git_oid("tree_oid", &member.tree_oid)?;
    validate_signing_identity(&member.release_signing_identity)?;
    match (&member.jeryu_url, &member.jeryu_slug) {
        (Some(url), Some(slug)) => {
            validate_jeryu_source(url, slug)?;
            if slug.rsplit('/').next() != Some(member.name.as_str()) {
                return Err(invalid(format!(
                    "{} Jeryu slug does not bind the member name",
                    member.name
                )));
            }
        }
        (None, None) => {
            return Err(invalid(format!(
                "{} lacks authenticated Jeryu URL/slug metadata",
                member.name
            )));
        }
        _ => {
            return Err(invalid(format!(
                "{} must provide both jeryu_url and jeryu_slug",
                member.name
            )));
        }
    }
    validate_files(&member.name, "lockfile", &member.lockfile, false)?;
    validate_files(&member.name, "artifact", &member.artifact, true)?;
    let lockfile_paths = member
        .lockfile
        .iter()
        .map(|file| file.path.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    if member
        .artifact
        .iter()
        .any(|file| lockfile_paths.contains(&file.path.to_ascii_lowercase()))
    {
        return Err(invalid(format!(
            "{} repeats a path across lockfile and artifact manifests",
            member.name
        )));
    }
    let expected_lockfile = if member.name == "bullet-portal" {
        "package-lock.json"
    } else {
        "Cargo.lock"
    };
    if member.lockfile.len() != 1 || member.lockfile[0].path != expected_lockfile {
        return Err(invalid(format!(
            "{} must bind exactly {expected_lockfile}",
            member.name
        )));
    }
    Ok(())
}

fn validate_files(
    member: &str,
    class: &str,
    files: &[LockedFile],
    permit_empty: bool,
) -> Result<(), CoordError> {
    if (!permit_empty && files.is_empty()) || files.len() > MAX_FILES_PER_MEMBER {
        return Err(invalid(format!(
            "{member} has an invalid number of {class} checksums"
        )));
    }
    let mut prior = None;
    let mut casefolded = BTreeSet::new();
    for file in files {
        validate_repository_path(&file.path)?;
        validate_digest("file digest", &file.digest)?;
        if prior.is_some_and(|path: &str| path >= file.path.as_str()) {
            return Err(invalid(format!(
                "{member} {class} paths must be unique and byte-sorted"
            )));
        }
        if !casefolded.insert(file.path.to_ascii_lowercase()) {
            return Err(invalid(format!(
                "{member} {class} paths collide under ASCII case folding"
            )));
        }
        prior = Some(file.path.as_str());
    }
    Ok(())
}

pub(crate) fn validate_jeryu_source(url: &str, slug: &str) -> Result<(), CoordError> {
    validate_slug(slug)?;
    if url.is_empty()
        || url.len() > 2048
        || !url.is_ascii()
        || url
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
        || url.contains(['\\', '?', '#'])
    {
        return Err(invalid("Jeryu source URL is malformed"));
    }
    let (remainder, loopback_only) = if let Some(rest) = url.strip_prefix("https://") {
        (rest, false)
    } else if let Some(rest) = url.strip_prefix("ssh://") {
        (rest, false)
    } else if let Some(rest) = url.strip_prefix("http://") {
        (rest, true)
    } else {
        return Err(invalid(
            "Jeryu source URL must use HTTPS/SSH, or HTTP on an explicit loopback host",
        ));
    };
    let (authority, path) = remainder
        .split_once('/')
        .ok_or_else(|| invalid("Jeryu source URL must contain a repository path"))?;
    if authority.is_empty() || authority.contains('@') {
        return Err(invalid(
            "Jeryu source URL must have a host and must not embed credentials",
        ));
    }
    if loopback_only && !valid_loopback_authority(authority) {
        return Err(invalid(
            "plain HTTP is permitted only for an explicit loopback host",
        ));
    }
    let expected_path = format!("git/{slug}.git");
    if path != expected_path {
        return Err(invalid(
            "Jeryu source URL path must be exactly /git/<jeryu_slug>.git",
        ));
    }
    Ok(())
}

pub(crate) fn validate_repository_path(path: &str) -> Result<(), CoordError> {
    if path.is_empty()
        || path.len() > 4096
        || !path.is_ascii()
        || path.starts_with('/')
        || path.ends_with('/')
        || path.contains('\\')
        || path.bytes().any(|byte| byte.is_ascii_control())
        || path.split('/').any(|segment| {
            segment.is_empty()
                || matches!(segment, "." | "..")
                || segment.eq_ignore_ascii_case(".git")
        })
    {
        return Err(invalid(format!("unsafe repository path: {path:?}")));
    }
    Ok(())
}

pub(crate) fn validate_tag(tag: &str) -> Result<(), CoordError> {
    if tag.len() > 128
        || !tag.starts_with('v')
        || !tag.as_bytes().get(1).is_some_and(u8::is_ascii_digit)
        || tag.ends_with(['.', '-'])
        || tag.contains("..")
        || tag
            .bytes()
            .any(|byte| !(byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-')))
    {
        return Err(invalid(
            "release tag must be a bounded v-prefixed ASCII version",
        ));
    }
    Ok(())
}

fn valid_loopback_authority(authority: &str) -> bool {
    if let Some(rest) = authority.strip_prefix("[::1]") {
        return valid_optional_port(rest);
    }
    for host in ["127.0.0.1", "localhost"] {
        if let Some(rest) = authority.strip_prefix(host) {
            return valid_optional_port(rest);
        }
    }
    false
}

fn valid_optional_port(rest: &str) -> bool {
    if rest.is_empty() {
        return true;
    }
    rest.strip_prefix(':').is_some_and(|port| {
        !port.is_empty()
            && port.bytes().all(|byte| byte.is_ascii_digit())
            && port.parse::<u16>().is_ok_and(|port| port != 0)
    })
}

fn validate_slug(slug: &str) -> Result<(), CoordError> {
    if slug.len() > 256
        || slug.split('/').count() != 2
        || slug.split('/').any(|part| {
            part.is_empty()
                || part.starts_with('.')
                || part.ends_with('.')
                || part.bytes().any(|byte| {
                    !(byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
                })
        })
    {
        return Err(invalid(
            "Jeryu slug must be a canonical owner/repository pair",
        ));
    }
    Ok(())
}

fn validate_atom(label: &str, value: &str, max: usize) -> Result<(), CoordError> {
    if value.is_empty()
        || value.len() > max
        || !value.is_ascii()
        || value
            .bytes()
            .any(|byte| !(byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'))
    {
        return Err(invalid(format!("{label} is not canonical lowercase ASCII")));
    }
    Ok(())
}

fn validate_git_oid(label: &str, oid: &str) -> Result<(), CoordError> {
    let valid = oid
        .strip_prefix("sha1:")
        .is_some_and(|hex| valid_lower_hex(hex, 40))
        || oid
            .strip_prefix("sha256:")
            .is_some_and(|hex| valid_lower_hex(hex, 64));
    if valid {
        Ok(())
    } else {
        Err(invalid(format!(
            "{label} is not an algorithm-tagged Git OID"
        )))
    }
}

fn validate_digest(label: &str, digest: &str) -> Result<(), CoordError> {
    if digest
        .strip_prefix("blake3:")
        .is_some_and(|hex| valid_lower_hex(hex, 64))
    {
        Ok(())
    } else {
        Err(invalid(format!("{label} is not a full BLAKE3 digest")))
    }
}

fn valid_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn validate_signing_identity(identity: &str) -> Result<(), CoordError> {
    let mut parts = identity.split('|');
    let principal = parts.next().unwrap_or_default();
    let algorithm = parts.next().unwrap_or_default();
    let fingerprint = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || principal.is_empty()
        || principal.len() > 256
        || principal
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control())
        || algorithm != "ed25519"
        || !fingerprint.starts_with("SHA256:")
        || fingerprint.len() > 128
        || fingerprint.bytes().any(|byte| {
            !(byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'+' | b'/' | b'='))
        })
    {
        return Err(invalid("release signing identity is malformed"));
    }
    Ok(())
}

fn invalid(reason: impl Into<String>) -> CoordError {
    CoordError::new("INVALID_FAMILY_LOCK", reason)
}

#[cfg(test)]
mod tests {
    use super::validate_jeryu_source;

    #[test]
    fn jeryu_source_accepts_the_canonical_smart_http_path() {
        for url in [
            "http://127.0.0.1:8787/git/root/bullet-kernel.git",
            "http://localhost:8787/git/root/bullet-kernel.git",
            "http://[::1]:8787/git/root/bullet-kernel.git",
            "https://jeryu.example/git/root/bullet-kernel.git",
            "ssh://jeryu.example/git/root/bullet-kernel.git",
        ] {
            validate_jeryu_source(url, "root/bullet-kernel")
                .unwrap_or_else(|error| panic!("canonical URL {url} was refused: {error}"));
        }
    }

    #[test]
    fn jeryu_source_refuses_derived_or_ambiguous_paths() {
        for url in [
            "https://jeryu.example/root/bullet-kernel.git",
            "https://jeryu.example/api/v3/git/root/bullet-kernel.git",
            "https://jeryu.example/git/root/bullet-kernel",
            "https://jeryu.example/git/root/bullet-kernel.git/",
            "https://jeryu.example/git/root/other.git",
            "https://user@jeryu.example/git/root/bullet-kernel.git",
            "http://jeryu.example/git/root/bullet-kernel.git",
        ] {
            assert!(
                validate_jeryu_source(url, "root/bullet-kernel").is_err(),
                "ambiguous or unsafe URL was admitted: {url}"
            );
        }
    }
}
