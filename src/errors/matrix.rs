use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct MatrixError;

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mal-formed matrix!")
    }
}

impl Error for MatrixError {}

#[derive(Debug, Clone)]
pub struct MatrixMultError;

impl fmt::Display for MatrixMultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of columns of matrix a must be equal to number of rows of matrix b!")
    }
}

impl Error for MatrixMultError {}
