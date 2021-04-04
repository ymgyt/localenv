use backtrace::Backtrace;

use std::{error, ffi::OsString, fmt, io, path::PathBuf};

pub trait ErrorContext {
    fn context(self, msg: impl Into<String>) -> Self;
}

impl<T> ErrorContext for Result<T, Error> {
    fn context(mut self, msg: impl Into<String>) -> Self {
        if let Err(ref mut err) = self {
            let trace = match Backtrace::new()
                .frames()
                .iter()
                .nth(5)
                .and_then(|frame| frame.symbols().first())
                .map(|symbol| (symbol.filename().map(|p| p.to_path_buf()), symbol.lineno()))
            {
                Some((filename, line)) => ErrorTrace {
                    filename,
                    line,
                    message: msg.into(),
                },
                None => ErrorTrace::new(msg.into()),
            };

            err.set_trace(trace);
        }
        self
    }
}

#[derive(Debug)]
struct ErrorTrace {
    filename: Option<PathBuf>,
    line: Option<u32>,
    message: String,
}

impl ErrorTrace {
    fn new(message: impl Into<String>) -> Self {
        Self {
            filename: None,
            line: None,
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    backtrace: Option<Backtrace>,
    context: Option<Vec<ErrorTrace>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    Internal(String),
    /// Configuration file not found.
    ConfigFileNotFound {
        path: PathBuf,
    },
    /// Failed to parse config file.
    ConfigFileParseFailed {
        yaml_err: serde_yaml::Error,
        path: PathBuf,
    },
    /// Invalid file permission.
    InvalidFilePermission {
        raw: String,
    },
    /// Command not found in $PATH.
    CommandNotFound {
        name: OsString,
        which_err: which::Error,
    },
    /// General unhandled I/O error.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;
        match self.kind() {
            Internal(message) => {
                write!(f, "error: {}", message)?;
            }
            ConfigFileNotFound { path, .. } => {
                write!(f, "config file not found: {}", path.display())?;
            }
            ConfigFileParseFailed { path, yaml_err, .. } => write!(
                f,
                "config file parse error: {} {}",
                path.display(),
                yaml_err
            )?,
            InvalidFilePermission { raw, .. } => {
                write!(
                    f,
                    "invalid file permission mode: {} (expect like that 664,700)",
                    raw
                )?;
            }
            CommandNotFound {
                which_err, name, ..
            } => {
                use std::os::unix::ffi::OsStrExt;
                write!(
                    f,
                    "command {} not found: {}",
                    String::from_utf8_lossy(name.as_bytes()),
                    which_err
                )?
            }
            Io(err) => {
                write!(f, "I/O error: {}", err)?;
            }
        }

        if let Some(context) = &self.context {
            write!(f, "\n\nContext:")?;

            for trace in context.iter() {
                let loc = format!(
                    "{}:{}",
                    trace
                        .filename
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default(),
                    trace.line.unwrap_or(0),
                );
                write!(f, "\n    {}\n        {}", loc, trace.message,)?;
            }
        }

        if let Some(bt) = &self.backtrace {
            Error::display_stacktrace(f, bt.frames())?;
        }

        Ok(())
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
    pub fn internal<T>(msg: &str) -> Result<T, Self> {
        Err(Error::from(ErrorKind::Internal(msg.to_owned())))
    }

    fn set_trace(&mut self, trace: ErrorTrace) {
        match &mut self.context {
            Some(ref mut context) => context.push(trace),
            None => {
                let v = vec![trace];
                self.context = Some(v);
            }
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    fn with_backtrace(kind: ErrorKind) -> Self {
        Self {
            kind,
            backtrace: Some(Backtrace::new()),
            context: None,
        }
    }

    fn display_stacktrace(
        f: &mut fmt::Formatter,
        frames: &[backtrace::BacktraceFrame],
    ) -> fmt::Result {
        let pkg_name = env!("CARGO_PKG_NAME");

        write!(f, "\n\nStacktrace:")?;

        for frame in frames {
            if let Some(symbol) = frame.symbols().first() {
                if let Some(filename) = symbol.filename() {
                    if filename.iter().any(|element| element.eq(pkg_name)) {
                        let line = symbol.lineno().unwrap_or(0);
                        let demangled = symbol
                            .name()
                            .unwrap_or_else(|| backtrace::SymbolName::new(&[]));

                        write!(f, "\n    {}:{} {} ", filename.display(), line, demangled)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl error::Error for Error {}
