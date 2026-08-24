use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("bullet-doctor-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove stale fixture");
    }
    let hub = root.join("bullet-farm");
    fs::create_dir_all(hub.join("scripts")).expect("fixture directories");
    fs::write(
        hub.join("Cargo.toml"),
        "[package]\nname='bullet-family'\nversion='0.0.0'\n",
    )
    .expect("Cargo fixture");
    fs::write(hub.join("scripts/setup.sh"), "#!/bin/sh\nexit 1\n").expect("setup fixture");
    fs::write(
        hub.join("family.lock"),
        concat!(
            "schema_version = \"2\"\n",
            "[[member]]\n",
            "name = \"bullet-farm\"\n",
            "commit_oid = \"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"\n",
            "[[member]]\n",
            "name = \"bullet-kernel\"\n",
            "commit_oid = \"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\"\n",
        ),
    )
    .expect("lock fixture");
    root
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(current).expect("read fixture") {
            let entry = entry.expect("entry");
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root).expect("relative").to_path_buf(),
                    fs::read(path).expect("file bytes"),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

#[test]
fn hub_only_doctor_is_json_read_only_and_honest() {
    let root = fixture_root("hub-only");
    let hub = root.join("bullet-farm");
    let before = snapshot(&root);
    let output = bullet_family::cli::run(
        [
            OsString::from("bullet-family"),
            OsString::from("doctor"),
            OsString::from("--json"),
        ],
        Ok(hub),
    )
    .expect("doctor report");
    let report: serde_json::Value = serde_json::from_str(&output).expect("JSON report");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["status"], "BLOCKED");
    let checks = report["checks"].as_array().expect("checks");
    for expected in ["hub_checkout", "source_metadata", "family_layout"] {
        let check = checks
            .iter()
            .find(|check| check["id"] == expected)
            .unwrap_or_else(|| panic!("missing {expected}"));
        assert_eq!(check["status"], "BLOCKED");
        assert!(
            check["repair"]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        );
    }
    assert_eq!(
        snapshot(&root),
        before,
        "doctor modified the hub-only clone"
    );
    fs::remove_dir_all(root).expect("remove fixture");
}

#[test]
fn doctor_rejects_non_json_and_invalid_roots() {
    let root = fixture_root("arguments");
    let hub = root.join("bullet-farm");
    let usage = bullet_family::cli::run(
        [OsString::from("bullet-family"), OsString::from("doctor")],
        Ok(hub.clone()),
    )
    .expect_err("--json is required");
    assert_eq!(usage.code(), "USAGE");
    let invalid = bullet_family::cli::run(
        [
            OsString::from("bullet-family"),
            OsString::from("--root"),
            root.join("missing").into_os_string(),
            OsString::from("doctor"),
            OsString::from("--json"),
        ],
        Ok(hub),
    )
    .expect_err("invalid explicit root");
    assert_eq!(invalid.code(), "COORD_IO_FAILED");
    fs::remove_dir_all(root).expect("remove fixture");
}
