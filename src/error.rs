use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TextkitError {
    Io(io::Error),
    InvalidArguments(String),
}

impl fmt::Display for TextkitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::InvalidArguments(message) => write!(f, "Invalid arguments: {message}"),
        }
    }
}

impl Error for TextkitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::InvalidArguments(_) => None,
        }
    }
}

impl From<io::Error> for TextkitError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
