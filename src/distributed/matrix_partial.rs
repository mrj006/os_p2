use crate::{errors::matrix::{MatrixError, MatrixMultError}, models::matrix};

pub fn matrix_cell_value(matrices: matrix::MatrixMultInput, row: usize, column: usize) -> Result<i64, Box<dyn std::error::Error>> {
    validate_matrices(&matrices)?;

    let matrix_a = &matrices.matrix_a.matrix;
    let matrix_b = &matrices.matrix_b.matrix;
    let mut res = 0;

    for j in 0..matrix_a[0].len() {
        res += matrix_a[row][j] * matrix_b[j][column];
    }

    Ok(res)
}

// TODO Could be done on master, once
pub fn validate_matrices(matrices: &matrix::MatrixMultInput) -> Result<(), Box<dyn std::error::Error>> {
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
