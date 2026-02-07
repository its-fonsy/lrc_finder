use std::env::VarError;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum LyricFinderError {
    StdIoError(String),
    OsString(String),
    ParseError(String),
    EnvError(String),
    InvalidRankListPositionError,
}

impl fmt::Display for LyricFinderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StdIoError(err) => write!(f, "{err}"),
            Self::OsString(err) => write!(f, "{err}"),
            Self::ParseError(err) => write!(f, "{err}"),
            Self::EnvError(err) => write!(f, "{err}"),
            Self::InvalidRankListPositionError => write!(f, "Invalid rank list position"),
        }
    }
}

impl From<VarError> for LyricFinderError {
    fn from(error: VarError) -> Self {
        Self::EnvError(error.to_string())
    }
}

impl From<ParseIntError> for LyricFinderError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseError(error.to_string())
    }
}

impl From<std::ffi::OsString> for LyricFinderError {
    fn from(error: std::ffi::OsString) -> Self {
        Self::OsString(error.into_string().expect("Error detecting OsString error"))
    }
}

impl From<std::io::Error> for LyricFinderError {
    fn from(error: std::io::Error) -> Self {
        Self::StdIoError(error.to_string())
    }
}

impl Error for LyricFinderError {}
