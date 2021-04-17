use serde::Deserialize;

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    error::ErrorKind,
    prelude::*,
    system::{FilePermission, Os},
};

#[derive(Deserialize, Debug)]
pub struct Filesystem {
    pub entries: Vec<FilesystemEntry>,
}

#[derive(Deserialize, Debug)]
pub enum FilesystemEntry {
    #[serde(rename = "symlink")]
    SymbolicLink(SymlinkEntry),
    #[serde(rename = "file")]
    File(FileEntry),
    #[serde(rename = "directory")]
    Directory(DirectoryEntry),
}

impl FilesystemEntry {
    pub fn condition(&self) -> Option<&FilesystemEntryCondition> {
        match self {
            FilesystemEntry::SymbolicLink(entry) => entry.base.condition.as_ref(),
            FilesystemEntry::File(entry) => entry.base.condition.as_ref(),
            FilesystemEntry::Directory(entry) => entry.base.condition.as_ref(),
        }
    }
    pub fn description(&self) -> &str {
        match self {
            FilesystemEntry::SymbolicLink(entry) => entry.base.description.as_str(),
            FilesystemEntry::File(entry) => entry.base.description.as_str(),
            FilesystemEntry::Directory(entry) => entry.base.description.as_str(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilesystemEntryBase {
    pub description: String,
    pub condition: Option<FilesystemEntryCondition>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilesystemEntryCondition {
    pub os: Option<Os>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymlinkEntry {
    #[serde(flatten)]
    pub base: FilesystemEntryBase,
    pub original_env_base: Option<String>,
    pub original_relative_path: Option<String>,
    pub link_env_base: Option<String>,
    pub link_relative_path: Option<String>,
}

impl SymlinkEntry {
    pub fn original_path(&self) -> PathBuf {
        let key = self
            .original_env_base
            .as_ref()
            .expect("original env base required");
        Path::new(&env::var_os(key).expect("env undefined")).join(
            self.original_relative_path
                .as_ref()
                .expect("original_relative_path"),
        )
    }

    pub fn link_path(&self) -> PathBuf {
        let key = self.link_env_base.as_ref().expect("link env base required");
        Path::new(&env::var_os(key).expect("env undefined")).join(
            self.link_relative_path
                .as_ref()
                .expect("link_relative_path"),
        )
    }

    pub fn description(&self) -> &str {
        self.base.description.as_str()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileEntry {
    #[serde(flatten)]
    pub base: FilesystemEntryBase,
    pub env_base: Option<String>,
    pub relative_path: Option<String>,
    pub content_from: PathBuf,
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

    pub fn description(&self) -> &str {
        self.base.description.as_str()
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
pub struct DirectoryEntry {
    #[serde(flatten)]
    pub base: FilesystemEntryBase,
}

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

            assert_eq!(
                entry.permission().ok(),
                Some(FilePermission::UnixMode(tc.1))
            );
        }
    }

    fn file_entry() -> FileEntry {
        FileEntry {
            base: FilesystemEntryBase {
                description: String::new(),
                condition: None,
            },
            env_base: None,
            relative_path: None,
            content_from: PathBuf::new(),
            raw_mode: String::new(),
        }
    }
}
