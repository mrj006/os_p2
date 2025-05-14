use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ImplementationError;

impl fmt::Display for ImplementationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse provided uri")
    }
}

impl Error for ImplementationError {}
