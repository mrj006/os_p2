use redis::{Commands, Connection, RedisResult};
use crate::models::matrix::{Matrix, MatrixPartialRes, MatrixMultInput};

/// Guarda un resultado parcial de una celda
/// Clave: "matrix:{job}:{x},{y}"
/// Valor: "{x},{y},{value}"
pub fn guardar_resultado_matriz(redis: &mut Connection, job:&str, resultado: &MatrixPartialRes) -> RedisResult<()> {
    let clave = format!("matrix:{}:{},{}", job, resultado.row, resultado.column);
    let valor = format!("{},{},{}", resultado.row, resultado.column, resultado.value);
    redis.set(clave, valor)
}

/// Obtiene todos los resultados parciales asociados a un job
/// Devuelve un Vec<MatrixPartialRes>
pub fn obtener_resultados_matriz(redis: &mut Connection, job: &str) -> RedisResult<Vec<MatrixPartialRes>> {
    let patron = format!("matrix:{}:*", job);
    let claves: Vec<String> = redis.keys(patron)?;
    let mut resultados = vec![];

    for clave in claves.iter() {
        if let Ok(valor) = redis.get::<_, String>(clave) {
            let partes: Vec<&str> = valor.split(',').collect();
            if partes.len() == 3 {
                if let (Ok(row), Ok(column), Ok(value)) = (
                    partes[0].parse::<usize>(),
                    partes[1].parse::<usize>(),
                    partes[2].parse::<i64>(),
                ) {
                    resultados.push(MatrixPartialRes {
                        row,
                        column,
                        value,
                    });
                }
            }
        }
    }

    Ok(resultados)
}

/// Borra todos los resultados parciales asociados a un job
pub fn borrar_resultados_matriz(redis: &mut Connection, job: &str) -> RedisResult<()> {
    let patron = format!("matrix:{}:*", job);
    let claves: Vec<String> = redis.keys(patron)?;
    for clave in claves {
        let _ = redis.del::<_, ()>(clave);
    }
    Ok(())
}


pub fn guardar_matrices_input(redis: &mut Connection, job: &str, input: &MatrixMultInput) -> RedisResult<()> {
    let clave = format!("input_matrices:{}", job);

    let string_a = input.matrix_a.matrix
        .iter()
        .map(|fila| fila.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
        .collect::<Vec<_>>()
        .join(";");

    let string_b = input.matrix_b.matrix
        .iter()
        .map(|fila| fila.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
        .collect::<Vec<_>>()
        .join(";");

    let contenido = format!("{}|{}", string_a, string_b);
    redis.set(clave, contenido)
}


pub fn leer_matrices_input(redis: &mut Connection, job: &str) -> RedisResult<MatrixMultInput> {
    let clave = format!("input_matrices:{}", job);
    let contenido: String = redis.get(clave)?;

    let mut partes = contenido.split('|');
    let string_a = partes.next().unwrap_or_default();
    let string_b = partes.next().unwrap_or_default();

    let vec_a = string_a
        .split(';')
        .map(|fila| fila.split(',')
        .filter_map(|v| v.parse::<i64>().ok())
        .collect::<Vec<i64>>())
        .collect::<Vec<_>>();

    let vec_b = string_b
        .split(';')
        .map(|fila| fila.split(',')
        .filter_map(|v| v.parse::<i64>().ok())
        .collect::<Vec<i64>>())
        .collect::<Vec<_>>();

    Ok(MatrixMultInput {
        matrix_a: Matrix { matrix: vec_a },
        matrix_b: Matrix { matrix: vec_b },
    })
}

/// Borra una matriz original de Redis según el job
pub fn borrar_matrices_input(redis: &mut Connection, job: &str) -> RedisResult<()> {
    let clave = format!("input_matrices:{}", job);
    redis.del(clave)
}

