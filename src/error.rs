use backtrace::Backtrace;

use std::{error, fmt, io, path::PathBuf};

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    backtrace: Option<Backtrace>,
}

#[derive(Debug)]
pub enum ErrorKind {
    #[allow(dead_code)]
    Internal(String),
    /// Configuration file not found.
    ConfigFileNotFound { path: PathBuf },
    /// Failed to parse config file.
    ConfigFileParseFailed {
        yaml_err: serde_yaml::Error,
        path: PathBuf,
    },
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
            Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::with_backtrace(kind)
    }
}

impl Error {
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
