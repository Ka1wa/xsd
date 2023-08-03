use std::error;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(roxmltree::Error),
    InvalidSchema
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidSchema => write!(f, "Invalid XSD schema"),
            Error::IOError(err) => write!(f, "{}", err),
            Error::ParseError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::InvalidSchema => None,
            Error::IOError(ref e) => Some(e),
            Error::ParseError(ref e) => Some(e),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(err: roxmltree::Error) -> Error {
        Error::ParseError(err)
    }
}
