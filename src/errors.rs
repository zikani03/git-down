use std::fmt::{Debug, Display, Formatter, Result};
use std::io;

#[derive(Debug)]
pub struct GitDownError {
    pub message: String,
}

impl Display for GitDownError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "GitDownError: {}", self.message)
    }
}

impl From<io::Error> for GitDownError {
    fn from(error: io::Error) -> Self {
        GitDownError {
            message: error.to_string(),
        }
    }
}
