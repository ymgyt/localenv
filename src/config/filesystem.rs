use serde::Deserialize;

use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Debug)]
pub struct Filesystem {
    base: Option<PathBuf>,
    entries: HashMap<String, FilesystemEntry>,
}

#[derive(Deserialize, Debug)]
pub enum FilesystemEntry {
    #[serde(rename = "symboliclink")]
    SymbolicLink(SymbolicLink),
    #[serde(rename = "file")]
    File(File),
    #[serde(rename = "directory")]
    Directory(Directory),
}

#[derive(Deserialize, Debug)]
pub struct SymbolicLink {}

#[derive(Deserialize, Debug)]
pub struct File {
    env_base: Option<String>,
    relative_path: Option<String>,
    content_from: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Directory {}
