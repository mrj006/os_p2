use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ServerError;

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "internal server error")
    }
}

impl Error for ServerError {}
