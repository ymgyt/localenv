use std::path::PathBuf;

use crate::{prelude::*, system::CommandApi};

pub struct Package {
    name: String,
    bin: String,
    version: semver::Version,
    local_path: Option<PathBuf>,
}

pub struct Cargo<Cmd> {
    cmd: Cmd,
}

impl<Cmd> Cargo<Cmd> where Cmd: CommandApi {
    pub fn list_installed_packages(&self) -> Result<Vec<Package>> {
        todo!()
    }
}
