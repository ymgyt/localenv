use serde::Deserialize;

use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Filesystem {
    base: Option<PathBuf>,
    entries: Vec<FilesystemEntry>,
}

#[derive(Deserialize, Debug)]
pub enum FilesystemEntry {
    SymbolicLink(SymbolicLink),
    File(File),
    Directory(Directory),
}

#[derive(Deserialize, Debug)]
pub struct SymbolicLink {}

#[derive(Deserialize, Debug)]
pub struct File {
    env_base: Option<String>,
    relative_path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Directory {}
