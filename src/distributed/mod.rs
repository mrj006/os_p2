pub mod count_partial;
pub mod count_total;
pub mod matrix_partial;
pub mod matrix_total;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::models::{count::CountJoinInput, matrix};

    use super::*;

    #[test]
    fn count_partial_success() {
        let file = fs::read_to_string("archivos/counttest.txt").unwrap();
        let res = count_partial::count_part_words(file.clone(), 0, 3);
        assert_eq!(10, res.unwrap());
        let res = count_partial::count_part_words(file.clone(), 1, 3);
        assert_eq!(7, res.unwrap());
        let res = count_partial::count_part_words(file.clone(), 2, 3);
        assert_eq!(6, res.unwrap());
    }

    #[test]
    #[should_panic]
    fn count_partial_part_error() {
        let file = fs::read_to_string("archivos/counttest.txt").unwrap();
        let _ = count_partial::count_part_words(file.clone(), 3, 3).unwrap();
    }

    #[test]
    fn count_total_sum_success() {
        let values = CountJoinInput { values: vec![10,7,6]};
        let res = count_total::count_join(values);
        assert_eq!(res, 23);
    }

    #[test]
    fn count_total_success() {
        let file = fs::read_to_string("archivos/DorianGray.txt").unwrap();
        let total: usize = 10;
        let mut values = Vec::<usize>::new();

        for i in 0..total {
            let res = count_partial::count_part_words(file.clone(), i, total).unwrap();
            values.push(res);
        }

        let values = CountJoinInput { values };
        let res = count_total::count_join(values);

        assert_eq!(res, 273);
    }

    #[test]
    #[should_panic]
    fn matrix_partial_invalid_matrix() {
        let matrix_a = matrix::Matrix { matrix: vec![vec![1,2,0], vec![3,4]] };
        let matrix_b = matrix::Matrix { matrix: vec![vec![5,6], vec![7,8]] };
        let matrices = matrix::MatrixMultInput { matrix_a, matrix_b };

        let _ = matrix_partial::matrix_cell_value(matrices, 0, 0).unwrap();
    }

    #[test]
    #[should_panic]
    fn matrix_partial_incompatible_matrices() {
        let matrix_a = matrix::Matrix { matrix: vec![vec![1,2,0], vec![3,4,0]] };
        let matrix_b = matrix::Matrix { matrix: vec![vec![5,6], vec![7,8]] };
        let matrices = matrix::MatrixMultInput { matrix_a, matrix_b };

        let _ = matrix_partial::matrix_cell_value(matrices, 0, 0).unwrap();
    }

    #[test]
    fn matrix_partial_success() {
        let matrix_a = matrix::Matrix { matrix: vec![vec![1,2], vec![3,4]] };
        let matrix_b = matrix::Matrix { matrix: vec![vec![5,6], vec![7,8]] };
        let matrices = matrix::MatrixMultInput { matrix_a, matrix_b };

        let res = matrix_partial::matrix_cell_value(matrices, 0, 0).unwrap();

        assert_eq!(res, 19);
    }

    #[test]
    fn matrix_total_success() {
        let matrix = matrix::Matrix { matrix: vec![vec![19,22], vec![43, 50]]};
        let value_a = matrix::MatrixPartialRes { row: 0, column: 0, value: 19 };
        let value_b = matrix::MatrixPartialRes { row: 0, column: 1, value: 22 };
        let value_c = matrix::MatrixPartialRes { row: 1, column: 0, value: 43 };
        let value_d = matrix::MatrixPartialRes { row: 1, column: 1, value: 50 };
        let values = vec![value_a, value_b, value_c, value_d];
        
        let res = matrix_total::matrix_multi_join(2, 2, values);

        assert_eq!(res, matrix);
    }
}
