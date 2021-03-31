mod filesystem;
pub use filesystem::Filesystem;

use serde::Deserialize;

use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "localenv")]
    version: String,

    required_envs: Vec<RequiredEnvEntry>,
    filesystem: Filesystem,
}

#[derive(Deserialize, Debug)]
pub struct RequiredEnvEntry {
    name: String,
    description: String,
    content_from: PathBuf,
}
