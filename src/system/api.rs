use std::{io, path::Path};

use crate::prelude::*;

pub trait Api {
    fn create_file<P, R>(&mut self, dest: P, content: R) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read;
}
