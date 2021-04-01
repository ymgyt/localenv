use serde::Deserialize;

use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Debug)]
pub struct Filesystem {
    pub base: Option<PathBuf>,
    pub entries: HashMap<String, FilesystemEntry>,
}

#[derive(Deserialize, Debug)]
pub enum FilesystemEntry {
    #[serde(rename = "symboliclink")]
    SymbolicLink(SymlinkEntry),
    #[serde(rename = "file")]
    File(FileEntry),
    #[serde(rename = "directory")]
    Directory(DirectoryEntry),
}

#[derive(Deserialize, Debug)]
pub struct SymlinkEntry {}

#[derive(Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub env_base: Option<String>,
    pub relative_path: Option<String>,
    pub content_from: PathBuf,
}

impl FileEntry {
    pub fn dest_path(&self) -> PathBuf {
        if let Some(key) = &self.env_base {
            Path::new(&env::var_os(key).expect("env undefined"))
                .join(self.relative_path.as_ref().expect("relative_path"))
        } else {
            unimplemented!("expect env_base")
        }
    }

    pub fn src_path(&self, root: impl AsRef<Path>) -> PathBuf {
        root.as_ref().join(self.content_from.as_path())
    }
}

#[derive(Deserialize, Debug)]
pub struct DirectoryEntry {}
