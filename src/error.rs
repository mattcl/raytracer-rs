use std::num::{ParseFloatError, ParseIntError};

pub type Result<T> = std::result::Result<T, RTError>;

/// RTError enumerates all possible errors returned by this library
#[derive(Debug)]
pub enum RTError {
    Error(String),
    InvalidGeo(String),

    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
}

impl std::error::Error for RTError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RTError::Error(_) => None,
            RTError::InvalidGeo(_) => None,
            RTError::ParseFloat(ref err) => Some(err),
            RTError::ParseInt(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for RTError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RTError::Error(ref msg) => {
                write!(f, "RTError: {}", msg)
            }
            RTError::InvalidGeo(ref msg) => {
                write!(f, "Invalid Geo Mesh: {}", msg)
            }
            RTError::ParseFloat(ref err) => err.fmt(f),
            RTError::ParseInt(ref err) => err.fmt(f),
        }
    }
}

impl From<ParseFloatError> for RTError {
    fn from(err: ParseFloatError) -> RTError {
        RTError::ParseFloat(err)
    }
}

impl From<ParseIntError> for RTError {
    fn from(err: ParseIntError) -> RTError {
        RTError::ParseInt(err)
    }
}
