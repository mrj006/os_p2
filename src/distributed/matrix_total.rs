use crate::distributed::matrix_partial::ResultadoCelda;

/// Construye la matriz resultante a partir de los resultados de las celdas individuales
pub fn construir_matriz_resultado(
    resultados: &[ResultadoCelda],
    filas: usize,
    columnas: usize,
) -> Vec<Vec<i32>> {
    let mut matriz_resultado = vec![vec![0; columnas]; filas];
    for resultado in resultados {
        if resultado.fila < filas && resultado.columna < columnas {
            matriz_resultado[resultado.fila][resultado.columna] = resultado.valor;
        }
    }
    matriz_resultado
}

/// Convierte la matriz resultante a formato JSON
pub fn matriz_a_json(matriz: &Vec<Vec<i32>>) -> String {
    serde_json::to_string(matriz).unwrap_or_else(|_| "[]".to_string())
}

/// Calcula las dimensiones de la matriz resultante
pub fn calcular_dimensiones_resultado(
    matriz_a: &Vec<Vec<i32>>,
    matriz_b: &Vec<Vec<i32>>,
) -> (usize, usize) {
    let filas_a = matriz_a.len();
    let columnas_b = if !matriz_b.is_empty() && !matriz_b[0].is_empty() {
        matriz_b[0].len()
    } else {
        0
    };
    (filas_a, columnas_b)
} 
