use std::{fmt, io, path::Path};

use crate::{
    prelude::*,
    system::{FilePermission, Os},
};

pub trait Api: FilesystemApi + CommandApi {
    fn os(&self) -> Os;

    fn display<D>(&self, msg: D)
    where
        D: fmt::Display;
}

pub trait FilesystemApi {
    fn create_file<P, R>(&mut self, dest: P, content: R, permission: FilePermission) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read;

    fn create_symbolic_link<P, Q>(&mut self, original: P, link: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>;
}

pub trait CommandApi {}
