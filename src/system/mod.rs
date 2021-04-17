mod api;
pub use api::{Api, CommandApi, FilesystemApi};

mod system;
pub use system::System;

mod os;
pub use os::Os;

mod command;
pub use command::{Command, resolve_binary_path};

#[derive(Debug, Clone, Copy,PartialEq)]
pub enum FilePermission {
    /// for unix family.
    UnixMode(u32),
    /// for windows.
    #[allow(dead_code)]
    Windows(),
}
