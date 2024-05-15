use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {
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
    fn from(value: std::io::Error) -> Self {
        dbg!(value);
        Error { message: String::from("IO Error") }
    }
}