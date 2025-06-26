use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct SlavesMissingError;

impl fmt::Display for SlavesMissingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No available slaves for assignment!")
    }
}

impl Error for SlavesMissingError {}

#[derive(Debug, Clone)]
pub struct SlaveFailedError;

impl fmt::Display for SlaveFailedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Slave failed during assignment!")
    }
}

impl Error for SlaveFailedError {}
