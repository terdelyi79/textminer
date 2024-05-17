use std::{fmt::{self, Display}, io::ErrorKind};

#[derive(Debug)]
pub struct Error {
    pub eof: bool,
    pub message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error
{
    fn from(error: std::io::Error) -> Self {        
        Error { eof: error.kind() == ErrorKind::UnexpectedEof, message: error.to_string() }
    }
}