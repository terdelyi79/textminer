use std::{error::Error, fmt::{self, Display}};

#[derive(Debug)]
pub struct Eof {}

impl Display for Eof {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eof")
    }
}

impl Error for Eof {}