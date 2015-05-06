//! Error handling and results.

use std::num::ParseIntError;
use std::fmt::{self, Display, Formatter};
use std::io;

/// Convenience alias for `Result` using our `Error` type.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Any error producible by the library.
#[derive(Debug)]
pub enum Error {
    /// An unknown or malformed initial world parameter was encountered.
    BadParameter,
    /// An attempt to parse an input as an integer failed.
    ParseInt(ParseIntError),
    UnexpectedLine,
    /// Eof was encountered before the game completed.
    UnexpectedEof,
    /// An IO error occurred.
    Io(io::Error),
    /// An unknown or malformed turn input command was encountered.
    UnknownCommand,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadParameter => "Bad world parameter",
            Error::ParseInt(ref e) => ::std::error::Error::description(e),
            Error::UnexpectedLine => "Unexpected line",
            Error::UnexpectedEof => "Unexpected Eof",
            Error::Io(ref e) => e.description(),
            Error::UnknownCommand => "Unknown command",
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> ::std::result::Result<(), fmt::Error> {
        write!(f, "{}", ::std::error::Error::description(self))
    }
}

impl ::std::convert::From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl ::std::convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
