use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ImplementationError;

impl fmt::Display for ImplementationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "request cannot be processed at this time")
    }
}

impl Error for ImplementationError {}
