use crate::models::matrix::{Matrix, MatrixPartialRes};

pub fn matrix_multi_join(rows: usize, columns: usize, values: Vec<MatrixPartialRes>) -> Matrix {
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; columns]; rows];
    
    for value in values {
        matrix[value.row][value.column] = value.value;
    }

    Matrix { matrix }
}
