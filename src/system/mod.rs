mod api;
pub use api::{Api,CommandApi};

mod system;
pub use system::System;

mod os;
pub use os::Os;

mod command;
pub use command::Command;

#[derive(Debug, Clone, Copy)]
pub enum FilePermission {
    /// for unix family.
    UnixMode(u32),
    /// for windows.
    #[allow(dead_code)]
    Windows(),
}
