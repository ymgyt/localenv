mod filesystem;
pub use filesystem::{FileEntry, Filesystem, FilesystemEntry, SymlinkEntry};

mod command;
pub use command::{Command, Installer};

use serde::Deserialize;
use tokio::fs;

use std::{
    io,
    path::{Path, PathBuf},
};

use crate::prelude::*;

const DEFAULT_CONFIG_FILE: &str = "localenv.yaml";

#[derive(Debug)]
pub struct Config {
    pub spec: Spec,
    pub root_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Spec {
    #[serde(rename = "localenv")]
    pub version: String,

    pub required_envs: Vec<RequiredEnvEntry>,
    pub commands: Vec<Command>,
    pub filesystem: Filesystem,
}

#[derive(Deserialize, Debug)]
pub struct RequiredEnvEntry {
    name: String,
    description: String,
}

impl Config {
    /// Load configuration from given dir.
    pub async fn load_from_dir(path: impl AsRef<Path>) -> Result<Self> {
        let dir_path = path
            .as_ref()
            .to_path_buf()
            .canonicalize()
            .expect("invalid path");
        let config_path = dir_path.join(DEFAULT_CONFIG_FILE);

        debug!(path = %(config_path.display()), "loading config file");

        let f = fs::File::open(&config_path)
            .await
            .map_err(|io_err| match io_err.kind() {
                io::ErrorKind::NotFound => ErrorKind::ConfigFileNotFound {
                    path: config_path.clone(),
                },
                _ => ErrorKind::Io(io_err),
            })?;

        let spec = serde_yaml::from_reader::<_, Spec>(f.into_std().await).map_err(|e| {
            ErrorKind::ConfigFileParseFailed {
                yaml_err: e,
                path: config_path,
            }
        })?;

        let cfg = Self {
            spec,
            root_dir: dir_path,
        };

        Ok(cfg)
    }
}
