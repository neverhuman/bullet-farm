use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use super::{SetupEnvironment, Toolchain};
use crate::setup::transaction::AdmittedRoot;

#[cfg(unix)]
#[test]
fn tool_admission_rejects_missing_relative_noncanonical_and_mismatched_inputs() {
    use std::os::unix::fs::{PermissionsExt, symlink};

    let fixture = fixture_root("tool-admission");
    let cargo = executable(
        &fixture,
        "cargo-real",
        "#!/bin/sh\nprintf 'cargo 1.97.1\\n'\n",
    );
    let node = executable(&fixture, "node-real", "#!/bin/sh\nprintf 'v26.1.0\\n'\n");
    let npm_cli = fixture.join("npm-cli.js");
    fs::write(&npm_cli, "fixture\n").expect("npm fixture");

    let error = Toolchain::admit(None, Some(&node), Some(&npm_cli))
        .expect_err("missing Cargo path must fail closed");
    assert_eq!(error.code(), "SETUP_TOOL_MISSING");

    let error = Toolchain::admit(Some(Path::new("cargo")), Some(&node), Some(&npm_cli))
        .expect_err("relative Cargo path must fail closed");
    assert_eq!(error.code(), "SETUP_TOOL_PATH_NOT_ABSOLUTE");

    let cargo_link = fixture.join("cargo-link");
    symlink(&cargo, &cargo_link).expect("Cargo symlink");
    let error = Toolchain::admit(Some(&cargo_link), Some(&node), Some(&npm_cli))
        .expect_err("non-canonical Cargo path must fail closed");
    assert_eq!(error.code(), "SETUP_TOOL_PATH_NOT_CANONICAL");

    let not_executable = fixture.join("cargo-data");
    fs::write(&not_executable, "cargo 1.97.1\n").expect("Cargo data");
    fs::set_permissions(&not_executable, fs::Permissions::from_mode(0o644))
        .expect("Cargo data permissions");
    let error = Toolchain::admit(Some(&not_executable), Some(&node), Some(&npm_cli))
        .expect_err("non-executable Cargo path must fail closed");
    assert_eq!(error.code(), "SETUP_TOOL_NOT_EXECUTABLE");

    let ambient_marker = fixture.join("ambient-cargo-executed");
    executable(
        &fixture,
        "cargo",
        &format!(
            "#!/bin/sh\nprintf executed > '{}'\n",
            ambient_marker.display()
        ),
    );
    let wrong = fs::canonicalize("/usr/bin/false").expect("canonical false executable");
    for attempt in 0..64 {
        let error = Toolchain::admit(Some(&wrong), Some(&node), Some(&npm_cli))
            .expect_err("wrong Cargo identity must fail closed");
        assert_eq!(
            error.code(),
            "SETUP_TOOL_IDENTITY_MISMATCH",
            "wrong-identity attempt {attempt} returned: {error}"
        );
    }
    assert!(!ambient_marker.exists(), "ambient Cargo executable ran");

    fs::remove_dir_all(fixture).expect("remove tool admission fixture");
}

#[cfg(unix)]
#[test]
fn admitted_tools_ignore_ambient_path_and_environment() {
    const CHILD: &str = "BULLET_SETUP_TOOL_TEST_CHILD";
    const ROOT: &str = "BULLET_SETUP_TOOL_TEST_ROOT";
    if std::env::var_os(CHILD).is_some() {
        admitted_tool_child(Path::new(
            &std::env::var_os(ROOT).expect("child fixture root"),
        ));
        return;
    }

    let fixture = fixture_root("tool-environment");
    let shim = fixture.join("shim");
    fs::create_dir(&shim).expect("shim directory");
    let shim_marker = fixture.join("shim-executed");
    for name in ["cargo", "node", "npm"] {
        executable(
            &shim,
            name,
            &format!(
                "#!/bin/sh\nprintf shim > '{}'\nexit 99\n",
                shim_marker.display()
            ),
        );
    }
    tool_fixtures(&fixture);

    let output = Command::new(std::env::current_exe().expect("test executable"))
        .args([
            "--exact",
            "setup::command::tests::admitted_tools_ignore_ambient_path_and_environment",
            "--nocapture",
        ])
        .env(CHILD, "1")
        .env(ROOT, &fixture)
        .env("BULLET_INSTALL_CANARY_SECRET", "must-not-leak")
        .env("PATH", &shim)
        .output()
        .expect("spawn isolated test child");
    assert!(
        output.status.success(),
        "isolated child failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!shim_marker.exists(), "ambient PATH shim executed");
    fs::remove_dir_all(fixture).expect("remove tool environment fixture");
}

#[cfg(unix)]
fn admitted_tool_child(fixture: &Path) {
    let toolchain = Toolchain::admit(
        Some(&fixture.join("cargo-real")),
        Some(&fixture.join("node-real")),
        Some(&fixture.join("npm-cli.js")),
    )
    .expect("admit exact fixture tools");
    let root = AdmittedRoot::open(fixture).expect("admit command fixture root");
    let environment = SetupEnvironment::create(&root, &toolchain).expect("isolated setup HOME");
    let home = environment.home_path().to_path_buf();
    toolchain
        .run_cargo(fixture, &["fetch", "--locked", "--offline"], &environment)
        .expect("run admitted Cargo");
    toolchain
        .run_npm(fixture, &["ci", "--offline"], &environment)
        .expect("run admitted npm");

    for observation in ["cargo-observed", "npm-observed"] {
        let observed = fs::read_to_string(fixture.join(observation)).expect("tool observation");
        assert!(observed.contains("canary=unset\n"), "{observed}");
        assert!(observed.contains("child=unset\n"), "{observed}");
        assert!(observed.contains("--offline"), "{observed}");
        assert!(!observed.contains("/shim"), "{observed}");
        assert!(observed.contains(&format!("home={}\n", home.display())));
    }
    environment.finish().expect("remove ephemeral setup HOME");
    assert!(!home.exists(), "ephemeral setup HOME survived drop");
}

#[cfg(target_os = "linux")]
#[test]
fn sealed_subjects_defeat_post_verification_path_swaps() {
    let fixture = fixture_root("tool-path-swap");
    let cargo = executable(
        &fixture,
        "cargo-real",
        concat!(
            "#!/bin/sh\n",
            "if [ \"${1-}\" = --version ]; then printf 'cargo 1.97.1\\n'; exit 0; fi\n",
            "printf admitted > cargo-admitted\n",
        ),
    );
    let node = executable(
        &fixture,
        "node-real",
        concat!(
            "#!/bin/sh\n",
            "if [ \"${1-}\" = --version ]; then printf 'v26.1.0\\n'; exit 0; fi\n",
            "exec /bin/sh \"$@\"\n",
        ),
    );
    let npm_cli = fixture.join("npm-cli.js");
    fs::write(
        &npm_cli,
        concat!(
            "if [ \"${1-}\" = --version ]; then printf '11.13.0\\n'; exit 0; fi\n",
            "printf admitted > npm-admitted\n",
        ),
    )
    .expect("npm CLI fixture");
    let toolchain = Toolchain::admit(Some(&cargo), Some(&node), Some(&npm_cli))
        .expect("admit exact fixture tools");
    let root = AdmittedRoot::open(&fixture).expect("admit command fixture root");
    let environment = SetupEnvironment::create(&root, &toolchain).expect("isolated setup HOME");

    let original_cargo = fixture.join("cargo-original");
    let error = toolchain
        .cargo
        .run_after_verify(&fixture, &["fetch"], &environment, || {
            fs::rename(&cargo, &original_cargo).expect("move verified Cargo path");
            executable(
                &fixture,
                "cargo-real",
                "#!/bin/sh\nprintf attacker > cargo-attacker\n",
            );
            Ok(())
        })
        .expect_err("post-verification Cargo swap must be reported");
    assert_eq!(error.code(), "SETUP_TOOL_CHANGED");
    assert_eq!(
        fs::read_to_string(fixture.join("cargo-admitted")).unwrap(),
        "admitted"
    );
    assert!(!fixture.join("cargo-attacker").exists());

    let original_npm = fixture.join("npm-original.js");
    let error = toolchain
        .npm
        .run_after_verify(&fixture, &["ci"], &environment, || {
            fs::rename(&npm_cli, &original_npm).expect("move verified npm CLI path");
            fs::write(&npm_cli, "printf attacker > npm-attacker\n")
                .expect("publish attacker npm CLI");
            Ok(())
        })
        .expect_err("post-verification npm companion swap must be reported");
    assert_eq!(error.code(), "SETUP_TOOL_CHANGED");
    assert_eq!(
        fs::read_to_string(fixture.join("npm-admitted")).unwrap(),
        "admitted"
    );
    assert!(!fixture.join("npm-attacker").exists());

    environment.finish().expect("remove ephemeral setup HOME");
    fs::remove_dir_all(fixture).expect("remove path-swap fixture");
}

#[cfg(unix)]
#[test]
fn setup_environment_detects_root_replacement_and_cleans_only_the_pinned_root() {
    let fixture = fixture_root("environment-root-replacement");
    let moved = fixture.with_extension("original");
    let _ = fs::remove_dir_all(&moved);
    tool_fixtures(&fixture);
    let cargo = fixture.join("cargo-real");
    let node = fixture.join("node-real");
    let npm_cli = fixture.join("npm-cli.js");
    let admit = |attempt| {
        Toolchain::admit(Some(&cargo), Some(&node), Some(&npm_cli)).unwrap_or_else(|error| {
            panic!("admit exact fixture tools on attempt {attempt}: {error}")
        })
    };
    let mut toolchain = admit(0);
    for attempt in 1..64 {
        toolchain = admit(attempt);
    }
    let root = AdmittedRoot::open(&fixture).expect("admit command fixture root");
    let environment = SetupEnvironment::create(&root, &toolchain).expect("isolated setup HOME");

    fs::rename(&fixture, &moved).expect("move admitted root");
    fs::create_dir(&fixture).expect("replacement root");
    fs::write(fixture.join("sentinel"), "preserve replacement\n").expect("replacement sentinel");
    let error = environment
        .verify()
        .expect_err("replaced environment root must fail closed");
    assert_eq!(error.code(), "SETUP_ROOT_REPLACED");
    drop(environment);
    assert_eq!(
        fs::read_to_string(fixture.join("sentinel")).unwrap(),
        "preserve replacement\n"
    );
    assert!(fs::read_dir(&moved).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(crate::setup::STAGING_PREFIX)
    }));
    fs::remove_dir_all(fixture).unwrap();
    fs::remove_dir_all(moved).unwrap();
}

#[cfg(unix)]
fn tool_fixtures(root: &Path) {
    executable(
        root,
        "cargo-real",
        concat!(
            "#!/bin/sh\n",
            "if [ \"${1-}\" = --version ]; then printf 'cargo 1.97.1\\n'; exit 0; fi\n",
            "printf 'canary=%s\\nchild=%s\\nargs=%s\\npath=%s\\nhome=%s\\n' \\\n",
            "  \"${BULLET_INSTALL_CANARY_SECRET-unset}\" \\\n",
            "  \"${BULLET_SETUP_TOOL_TEST_CHILD-unset}\" \"$*\" \"$PATH\" \"$HOME\" \\\n",
            "  > cargo-observed\n",
        ),
    );
    executable(
        root,
        "node-real",
        concat!(
            "#!/bin/sh\n",
            "if [ \"${1-}\" = --version ]; then printf 'v26.1.0\\n'; exit 0; fi\n",
            "if [ \"${2-}\" = --version ]; then printf '11.13.0\\n'; exit 0; fi\n",
            "printf 'canary=%s\\nchild=%s\\nargs=%s\\npath=%s\\nhome=%s\\n' \\\n",
            "  \"${BULLET_INSTALL_CANARY_SECRET-unset}\" \\\n",
            "  \"${BULLET_SETUP_TOOL_TEST_CHILD-unset}\" \"$*\" \"$PATH\" \"$HOME\" \\\n",
            "  > npm-observed\n",
        ),
    );
    fs::write(root.join("npm-cli.js"), "fixture\n").expect("npm CLI fixture");
}

#[cfg(unix)]
fn executable(root: &Path, name: &str, contents: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let path = root.join(name);
    let publishing = root.join(format!(".{name}.publishing"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&publishing)
        .expect("create private executable fixture");
    file.write_all(contents.as_bytes())
        .expect("write executable fixture");
    file.set_permissions(fs::Permissions::from_mode(0o755))
        .expect("fixture permissions");
    file.sync_all().expect("sync executable fixture");
    drop(file);
    fs::hard_link(&publishing, &path).expect("publish executable fixture without replacement");
    fs::remove_file(&publishing).expect("remove private executable fixture name");
    fs::File::open(root)
        .expect("open executable fixture directory")
        .sync_all()
        .expect("sync executable fixture directory");
    path
}

fn fixture_root(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "bullet-setup-command-{name}-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("command fixture root");
    root
}
