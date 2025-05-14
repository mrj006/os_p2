use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ParseUriError;

impl fmt::Display for ParseUriError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse provided uri")
    }
}

impl Error for ParseUriError {}
