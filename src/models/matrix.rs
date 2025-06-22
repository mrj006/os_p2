use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix {
    pub matrix: Vec<Vec<i64>>
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.matrix.len() != other.matrix.len() {
            return false;
        }
        
        let rows = self.matrix.len();
        let columns = self.matrix[0].len();
        
        for i in 0..rows {
            if self.matrix[i].len() != other.matrix[i].len() {
                return false;
            }

            for j in 0..columns {
                if self.matrix[i][j] != other.matrix[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixMultInput {
    pub matrix_a: Matrix,
    pub matrix_b: Matrix
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixPartialRes {
    pub row: usize,
    pub column: usize,
    pub value: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatrixJoinInput {
    pub values: Vec<MatrixPartialRes>
}
