pub mod count_partial;
pub mod count_total;
pub mod matrix_partial;
pub mod matrix_total;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn count() {
        let file = fs::read_to_string("archivos/counttest.txt").unwrap();
        let res = count_partial::contar_palabras(file.clone(), 0, 3);
        assert_eq!(10, res.unwrap());
        let res = count_partial::contar_palabras(file.clone(), 1, 3);
        assert_eq!(7, res.unwrap());
        let res = count_partial::contar_palabras(file.clone(), 2, 3);
        assert_eq!(6, res.unwrap());
    }
}