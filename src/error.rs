//! Error types

use std::fmt;

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context>);

impl Error {
    /// Create a new error with the given source
    pub fn new(kind: ErrorKind, source: impl Into<BoxError>) -> Self {
        Error(Box::new(Context {
            kind,
            source: Some(source.into()),
        }))
    }

    /// Get the kind of error
    pub fn kind(&self) -> &ErrorKind {
        &self.0.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind())?;

        if let Some(source) = &self.0.source {
            write!(f, ": {}", source)?;
        }

        Ok(())
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error(Box::new(Context { kind, source: None }))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0
            .source
            .as_ref()
            .map(|source| source.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// Kinds of errors
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Argument errors
    Argument,

    /// Build errors
    Build,

    /// Cargo-related errors
    Cargo,

    /// Errors relating to Docker images
    DockerImage,

    /// Path-related errors
    Path,

    /// Toolchain errors
    Toolchain,

    /// Errors relating to workspaces
    Workspace,
}

impl ErrorKind {
    /// Create an error of this kind, with another error as the source
    pub fn source(&self, error: impl Into<BoxError>) -> Error {
        Error::new(self.clone(), error)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ErrorKind::Argument => "invalid argument",
            ErrorKind::Build => "build error",
            ErrorKind::Cargo => "cargo error",
            ErrorKind::DockerImage => "docker image error",
            ErrorKind::Path => "invalid path",
            ErrorKind::Toolchain => "toolchain error",
            ErrorKind::Workspace => "workspace error",
        })
    }
}

/// Boxed error type used as an error source
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Error context
#[derive(Debug)]
struct Context {
    /// Kind of error
    kind: ErrorKind,

    /// Source of error
    source: Option<BoxError>,
}
