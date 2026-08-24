use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use super::{SetupEnvironment, Toolchain};

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

    let wrong = executable(
        &fixture,
        "wrong-cargo",
        "#!/bin/sh\nprintf 'npm 11.13.0\\n'\n",
    );
    let error = Toolchain::admit(Some(&wrong), Some(&node), Some(&npm_cli))
        .expect_err("wrong Cargo identity must fail closed");
    assert_eq!(
        error.code(),
        "SETUP_TOOL_IDENTITY_MISMATCH",
        "unexpected tool-admission error: {error}"
    );

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
    let environment = SetupEnvironment::create(fixture, &toolchain).expect("isolated setup HOME");
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
    drop(environment);
    assert!(!home.exists(), "ephemeral setup HOME survived drop");
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
    fs::write(&path, contents).expect("write executable fixture");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("fixture permissions");
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
