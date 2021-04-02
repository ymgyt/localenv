use backtrace::Backtrace;

use std::{error, fmt, io, path::PathBuf};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    backtrace: Option<Backtrace>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Internal(String),
    /// Configuration file not found.
    ConfigFileNotFound { path: PathBuf },
    /// Failed to parse config file.
    ConfigFileParseFailed {
        yaml_err: serde_yaml::Error,
        path: PathBuf,
    },
    /// Invalid file permission.
    InvalidFilePermission{ raw: String },
    /// General unhandled I/O error.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.kind() {
            Internal(message) => write!(f, "error: {}", message),
            ConfigFileNotFound { path, .. } => {
                write!(f, "config file not found: {}", path.display())
            }
            ConfigFileParseFailed { path, yaml_err, .. } => write!(
                f,
                "config file parse error: {} {}",
                path.display(),
                yaml_err
            ),
            InvalidFilePermission{ raw, ..} => {
                write!(f, "invalid file permission mode: {} (expect like that 664,700)", raw)
            }
            Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::with_backtrace(kind)
    }
}

macro_rules! impl_from_error {
    ($e:path, $kind:path) => {
        impl From<$e> for Error {
            fn from(err: $e) -> Self {
                Error::from($kind(err))
            }
        }
    };
}

impl_from_error!(io::Error, ErrorKind::Io);

impl Error {
    pub fn internal<T>(msg: &str) -> Result<T,Self> {
        Err(Error::from(ErrorKind::Internal(msg.to_owned())))
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    fn with_backtrace(kind: ErrorKind) -> Self {
        Self {
            kind,
            backtrace: Some(Backtrace::new()),
        }
    }
}

impl error::Error for Error {}
