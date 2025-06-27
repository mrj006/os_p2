use redis::RedisResult;

use crate::models::matrix::{Matrix, MatrixPartialRes, MatrixMultInput};

use super::connection;

/// Stores a cell value result of a matrices multiplication, tied to a job ID
pub fn add_matrix_part_res(job:&str, cell: MatrixPartialRes) -> RedisResult<()> {
    let key = format!("matrix:{}:{},{}", job, cell.row, cell.column);
    let value = serde_json::to_string(&cell).unwrap();
    connection::add_data_to_redis(key, value)
}

/// Gets all cell results for a given job ID
pub fn get_all_matrix_part_res(job: &str) -> Result<Vec<MatrixPartialRes>, Box<dyn std::error::Error>> {
    let pattern = format!("matrix:{}:*", job);
    let values = connection::get_values_from_redis(pattern)?;
    let mut res: Vec<MatrixPartialRes> = vec![];

    for value in values {
        let value = serde_json::from_str::<MatrixPartialRes>(&value).unwrap();
        res.push(value);
    }

    Ok(res)
}

/// Removes all cell results of a given job ID
pub fn remove_all_matrix_part_res(job: &str) -> RedisResult<()> {
    let pattern = format!("matrix:{}:*", job);
    connection::remove_keys_from_redis(pattern)
}


pub fn add_matrices_input(job: &str, matrices: &MatrixMultInput) -> RedisResult<()> {
    let key = format!("matrices_input:{}", job);
    let value = serde_json::to_string(matrices).unwrap();
    connection::add_data_to_redis(key, value)
}


pub fn get_matrices_input(job: &str) -> Result<MatrixMultInput, Box<dyn std::error::Error>> {
    let key = format!("matrices_input:{}", job);
    let value: String = connection::get_value_from_redis(key)?;
    let matrices = serde_json::from_str::<MatrixMultInput>(&value).unwrap();

    Ok(matrices)
}

pub fn remove_matrices_input(job: &str) -> RedisResult<()> {
    let key = format!("matrices_input:{}", job);
    connection::remove_key_from_redis(key)
}

pub fn add_matrix_res(job: &str, matrix: &Matrix) -> RedisResult<()> {
    let key = format!("matrices_output:{}", job);
    let value = serde_json::to_string(matrix).unwrap();
    connection::add_data_to_redis(key, value)
}

pub fn get_matrix_res(job: &str) -> Result<Matrix, Box<dyn std::error::Error>> {
    let key = format!("matrices_output:{}", job);
    let value = connection::get_value_from_redis(key)?;
    let value = serde_json::from_str::<Matrix>(&value).unwrap();
    Ok(value)
}

pub fn remove_matrix_res(job: &str) -> RedisResult<()> {
    let key = format!("matrices_output:{}", job);
    connection::remove_key_from_redis(key)
}

pub fn remove_job(job: &str) -> RedisResult<()> {
    let _ = remove_matrices_input(job)?;
    let _ = remove_all_matrix_part_res(job)?;
    remove_matrix_res(job)
}