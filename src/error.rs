//! error representations

#[derive(Debug)]
pub enum ErrorKind {
    ParseError(std::string::String),
    IoError(std::io::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Error {
        Error{kind: kind}
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error {kind: ErrorKind::IoError(e)}
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Error {
        Error {kind: ErrorKind::ParseError(format!("{:?}", e))}
    }
}

impl std::convert::From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Error {
        Error {kind: ErrorKind::ParseError(format!("{:?}", e))}
    }
}

impl std::convert::From<std::string::ParseError> for Error {
    fn from(e: std::string::ParseError) -> Error {
        Error {kind: ErrorKind::ParseError(format!("{:?}", e))}
    }
}

pub type Result<T> = std::result::Result<T, Error>;
