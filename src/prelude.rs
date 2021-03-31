/// Logging utilities.
pub use tracing::{debug, error, info, trace, warn};

/// A specialized Result type for this crate.
pub type Result<T, E = crate::error::Error> = std::result::Result<T, E>;

pub use crate::error::{Error, ErrorKind};
