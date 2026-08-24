use std::{
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use super::Toolchain;
use crate::{coord::CoordError, setup::STAGING_PREFIX};

const HOME_COMPONENT: &str = "home";

#[derive(Debug)]
pub(in crate::setup) struct SetupEnvironment {
    family_root: PathBuf,
    root: PathBuf,
    home: PathBuf,
    cargo_home: PathBuf,
    npm_cache: PathBuf,
    temporary: PathBuf,
    trusted_path: OsString,
}

impl SetupEnvironment {
    pub(in crate::setup) fn create(
        family_root: &Path,
        toolchain: &Toolchain,
    ) -> Result<Self, CoordError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| CoordError::new("CLOCK_BEFORE_EPOCH", error.to_string()))?
            .as_nanos();
        for sequence in 0..64 {
            let root = family_root.join(format!(
                "{STAGING_PREFIX}{HOME_COMPONENT}.{}.{}.{}",
                std::process::id(),
                now,
                sequence
            ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    return Self::initialize(
                        family_root,
                        root,
                        toolchain.trusted_path().to_owned(),
                    );
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(CoordError::io(error)),
            }
        }
        Err(CoordError::new(
            "STAGING_COLLISION",
            "could not allocate an ephemeral setup HOME",
        ))
    }

    fn initialize(
        family_root: &Path,
        root: PathBuf,
        trusted_path: OsString,
    ) -> Result<Self, CoordError> {
        let home = root.join("home");
        let cargo_home = root.join("cargo");
        let npm_cache = root.join("npm-cache");
        let temporary = root.join("tmp");
        for path in [&home, &cargo_home, &npm_cache, &temporary] {
            if let Err(error) = fs::create_dir(path) {
                let _ = fs::remove_dir_all(&root);
                return Err(CoordError::io(error));
            }
        }
        Ok(Self {
            family_root: family_root.to_path_buf(),
            root,
            home,
            cargo_home,
            npm_cache,
            temporary,
            trusted_path,
        })
    }

    pub(super) fn apply(&self, command: &mut Command) {
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("CARGO_HOME", &self.cargo_home)
            .env("npm_config_cache", &self.npm_cache)
            .env("TMPDIR", &self.temporary)
            .env("PATH", &self.trusted_path)
            .env("LC_ALL", "C")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_NO_REPLACE_OBJECTS", "1");
    }

    #[cfg(test)]
    pub(super) fn home_path(&self) -> &Path {
        &self.home
    }
}

impl Drop for SetupEnvironment {
    fn drop(&mut self) {
        let safe_name = self
            .root
            .file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|name| name.starts_with(&format!("{STAGING_PREFIX}{HOME_COMPONENT}.")));
        if safe_name && self.root.parent() == Some(self.family_root.as_path()) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}
