use std::{fs, io, path::Path};

use crate::{config::FilePermission, prelude::*, system};

pub struct System {}

impl system::Api for System {
    #[cfg(target_family = "unix")]
    fn create_file<P, R>(
        &mut self,
        dest: P,
        mut content: R,
        permission: FilePermission,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read,
    {
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

        let mode = match permission {
            FilePermission::UnixMode(mode) => mode,
            _ => return Error::internal("could not get unix file permission"),
        };

        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true) // overwrite if exists
            .create(true) // create if not exists.
            .mode(mode)
            .open(dest)?;

        io::copy(&mut content, &mut file)?;

        // set permission explicitly because when file already exists, the mode specification at open does not.
        let mut perm = file.metadata()?.permissions();
        perm.set_mode(mode);
        file.set_permissions(perm)?;

        Ok(())
    }
}

impl<'a, T: system::Api> system::Api for &'a mut T {
    fn create_file<P, R>(&mut self, dest: P, content: R,permission: FilePermission) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read,
    {
        (**self).create_file(dest, content, permission)
    }
}

impl System {
    pub fn new() -> Self {
        Self {}
    }
}
