use crate::models::matrix;

pub fn matrix_cell_value(matrices: matrix::MatrixMultInput, row: usize, column: usize) -> i64 {
    let matrix_a = &matrices.matrix_a.matrix;
    let matrix_b = &matrices.matrix_b.matrix;
    let mut res = 0;

    for j in 0..matrix_a[0].len() {
        res += matrix_a[row][j] * matrix_b[j][column];
    }

    res
}
