use std::{fmt, fs, io, path::Path};

use crate::{
    prelude::*,
    system::{self, FilePermission, Os},
};

pub struct System {
    os: Os,
}

impl system::Api for System {
    fn os(&self) -> Os {
        self.os
    }

    fn display<D>(&self, msg: D)
        where
            D: fmt::Display,
    {
        println!("{}", msg);
    }
}

impl <'a, T: system::Api> system::Api for &'a mut T {
    fn os(&self) -> Os {
        (**self).os()
    }

    fn display<D>(&self, msg: D)
        where
            D: fmt::Display,
    {
        (**self).display(msg)
    }
}

impl system::FilesystemApi for System {
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

    #[cfg(target_family = "unix")]
    fn create_symbolic_link<P, Q>(&mut self, original: P, link: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        return match std::os::unix::fs::symlink(&original, &link) {
            Ok(_) => Ok(()),
            Err(io_err) => {
                if let io::ErrorKind::AlreadyExists = io_err.kind() {
                    // TODO: delegate api implementation.
                    debug!("{} already exists, try removing", link.as_ref().display());
                    fs::remove_file(&link)?;
                    // TODO: care infinite loop.
                    return self.create_symbolic_link(original, link);
                }
                Err(io_err.into())
            }
        };
    }

}

impl<'a, T: system::Api> system::FilesystemApi for &'a mut T {
    fn create_file<P, R>(&mut self, dest: P, content: R, permission: FilePermission) -> Result<()>
    where
        P: AsRef<Path>,
        R: io::Read,
    {
        (**self).create_file(dest, content, permission)
    }

    fn create_symbolic_link<P, Q>(&mut self, original: P, link: Q) -> Result<()>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        (**self).create_symbolic_link(original, link)
    }
}

impl system::CommandApi for System {}

impl<'a, T: system::CommandApi> system::CommandApi for &'a mut T {

}

impl System {
    pub fn new() -> Self {
        Self { os: Os::detect() }
    }
}
