use std::{ffi::OsStr, path::PathBuf};

use crate::{prelude::*, system::CommandApi};

pub struct Command {}

impl CommandApi for Command {}

pub fn resolve_binary_path(path: impl AsRef<OsStr>) -> Result<PathBuf> {
    which::which(&path).map_err(|err| {
        Error::from(ErrorKind::CommandNotFound {
            which_err: err,
            name: path.as_ref().to_owned(),
        })
    })
}
