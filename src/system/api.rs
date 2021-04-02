use std::{io, path::Path};

use crate::{prelude::*, system::{Os,FilePermission}};

pub trait Api {
    fn create_file<P, R>(&mut self, dest: P, content: R, permission: FilePermission) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read;

    fn create_symbolic_link<P, Q>(&mut self, original: P, link: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>;

    fn os(&self) -> Os;
}
