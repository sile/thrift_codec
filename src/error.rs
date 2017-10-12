use std;
use trackable::error::TrackableError;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt};

/// This crate specific error type.
#[derive(Debug, Clone)]
pub struct Error(TrackableError<ErrorKind>);
derive_traits_for_trackable_error_newtype!(Error, ErrorKind);
impl From<std::io::Error> for Error {
    fn from(f: std::io::Error) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(f: std::string::FromUtf8Error) -> Self {
        ErrorKind::InvalidInput.cause(f).into()
    }
}

/// The list of the possible error kinds
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Input data is invalid.
    InvalidInput,

    /// Other errors (e.g., I/O error).
    Other,
}
impl TrackableErrorKind for ErrorKind {}
