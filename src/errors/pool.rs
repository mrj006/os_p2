use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct PoolError;

impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to initialize thread pool")
    }
}

impl Error for PoolError {}
