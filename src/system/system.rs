use std::{fs, io, path::Path};

use crate::{prelude::*, system};

pub struct System {}

impl system::Api for System {
    fn create_file<P, R>(&mut self, dest: P, mut content: R) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read,
    {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true) // overwrite if exists
            .create(true) // create if not exists.
            .open(dest)?;

        io::copy(&mut content, &mut file)?;

        Ok(())
    }
}

impl<'a, T: system::Api> system::Api for &'a mut T {
    fn create_file<P, R>(&mut self, dest: P, content: R) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read,
    {
        (**self).create_file(dest, content)
    }
}

impl System {
    pub fn new() -> Self {
        Self {}
    }
}
