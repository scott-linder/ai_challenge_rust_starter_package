use std::num::ParseIntError;
use std::fmt::{self, Display, Formatter};
use std::io;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadParameter,
    ParseInt(ParseIntError),
    UnexpectedLine,
    UnexpectedEof,
    Io(io::Error),
    UnknownCommand,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadParameter => "Bad world parameter",
            Error::ParseInt(ref e) => e.description(),
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

impl ::std::error::FromError<ParseIntError> for Error {
    fn from_error(err: ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl ::std::error::FromError<io::Error> for Error {
    fn from_error(err: io::Error) -> Error {
        Error::Io(err)
    }
}
