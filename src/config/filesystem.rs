use serde::Deserialize;

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{error::ErrorKind, prelude::*};

#[derive(Debug,Clone,Copy)]
pub enum FilePermission {
    /// for unix family.
    UnixMode(u32),
    /// for windows.
    #[allow(dead_code)]
    Windows(),
}

#[derive(Deserialize, Debug)]
pub struct Filesystem {
    pub entries: Vec<FilesystemEntry>,
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
    pub description: String,
    #[serde(rename(deserialize = "mode"))]
    pub raw_mode: String,
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

    #[cfg(target_family = "unix")]
    pub fn permission(&self) -> Result<FilePermission> {
        u32::from_str_radix(&self.raw_mode, 8)
            .map_err(|_| {
                ErrorKind::InvalidFilePermission {
                    raw: self.raw_mode.clone(),
                }
                .into()
            })
            .map(FilePermission::UnixMode)
    }
}

#[derive(Deserialize, Debug)]
pub struct DirectoryEntry {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_family = "unix")]
    fn file_entry_mode() {
        for tc in vec![
            // input, expect
            ("666", 0o666),
            ("0666", 0o666),
            ("700", 0o700),
        ] {
            let mut entry = file_entry();
            entry.raw_mode = tc.0.to_owned();

            assert_eq!(entry_1.permission().ok(), Some(FilePermission::UnixMode(tc.1)));
        }
    }

    fn file_entry() -> FileEntry {
        FileEntry {
            env_base: None,
            relative_path: None,
            content_from: PathBuf::new(),
            description: String::new(),
            raw_mode: String::new(),
        }
    }
}
