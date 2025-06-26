use crate::errors::matrix::{MatrixError, MatrixMultError};
use crate::models::matrix::{Matrix, MatrixMultInput, MatrixPartialRes};

pub fn matrix_multi_join(rows: usize, columns: usize, values: Vec<MatrixPartialRes>) -> Matrix {
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; columns]; rows];
    
    for value in values {
        matrix[value.row][value.column] = value.value;
    }

    Matrix { matrix }
}

pub fn validate_matrices(matrices: &MatrixMultInput) -> Result<(), Box<dyn std::error::Error>> {
    let matrix_a = &matrices.matrix_a.matrix;
    let matrix_b = &matrices.matrix_b.matrix;

    for row in matrix_a {
        if row.len() != matrix_a[0].len() {
            return Err(Box::new(MatrixError));
        }
    }

    for row in matrix_b {
        if row.len() != matrix_b[0].len() {
            return Err(Box::new(MatrixError));
        }
    }

    if matrix_a[0].len() != matrix_b.len() {
        return Err(Box::new(MatrixMultError));
    }

    Ok(())
}
