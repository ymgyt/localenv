mod api;
pub use api::Api;

mod system;
pub use system::System;

mod os;
pub use os::Os;

#[derive(Debug,Clone,Copy)]
pub enum FilePermission {
    /// for unix family.
    UnixMode(u32),
    /// for windows.
    #[allow(dead_code)]
    Windows(),
}
