use std::{io, path::Path};

use crate::{prelude::*, config::FilePermission};

pub trait Api {
    fn create_file<P, R>(&mut self, dest: P, content: R,permission: FilePermission) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read;
}
