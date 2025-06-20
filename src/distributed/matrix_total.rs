use serde::{Deserialize, Serialize};
use crate::distributed::matrix_partial::ResultadoCelda;

#[derive(Debug, Serialize, Deserialize)]
pub struct CeldaTarea {
    pub fila: usize,
    pub columna: usize,
    pub matriz_a: Vec<Vec<i32>>,
    pub matriz_b: Vec<Vec<i32>>,
}

/// Decodifica una cadena URL-encoded
pub fn url_decode(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let (Some(a), Some(b)) = (chars.next(), chars.next()) {
                if let Ok(byte) = u8::from_str_radix(&format!("{}{}", a, b), 16) {
                    result.push(byte as char);
                } else {
                    result.push(ch);
                    result.push(a);
                    result.push(b);
                }
            } else {
                result.push(ch);
            }
        } else if ch == '+' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Parsea matrices desde strings JSON con decodificación URL
pub fn parse_matrices(matriz_a_str: &str, matriz_b_str: &str) -> Result<(Vec<Vec<i32>>, Vec<Vec<i32>>), serde_json::Error> {
    // Intentar parsear directamente
    let matriz_a: Result<Vec<Vec<i32>>, _> = serde_json::from_str(matriz_a_str);
    let matriz_b: Result<Vec<Vec<i32>>, _> = serde_json::from_str(matriz_b_str);

    if let (Ok(a), Ok(b)) = (matriz_a, matriz_b) {
        return Ok((a, b));
    }

    // Si falla, intentar decodificar y parsear de nuevo
    let matriz_a_decoded = url_decode(matriz_a_str);
    let matriz_b_decoded = url_decode(matriz_b_str);

    let matriz_a: Vec<Vec<i32>> = serde_json::from_str(&matriz_a_decoded)?;
    let matriz_b: Vec<Vec<i32>> = serde_json::from_str(&matriz_b_decoded)?;

    Ok((matriz_a, matriz_b))
}

/// Calcula el valor de una celda específica en la matriz resultante
/// de la multiplicación de matriz_a × matriz_b
pub fn calcular_celda_matriz(tarea: &CeldaTarea) -> ResultadoCelda {
    let fila = tarea.fila;
    let columna = tarea.columna;
    let matriz_a = &tarea.matriz_a;
    let matriz_b = &tarea.matriz_b;
    
    // Verificar que las matrices son compatibles para multiplicación
    if matriz_a.is_empty() || matriz_b.is_empty() || matriz_a[0].len() != matriz_b.len() {
        return ResultadoCelda {
            fila,
            columna,
            valor: 0,
        };
    }
    
    // Calcular el producto punto de la fila i de A con la columna j de B
    let mut valor = 0;
    for k in 0..matriz_a[0].len() {
        valor += matriz_a[fila][k] * matriz_b[k][columna];
    }
    
    ResultadoCelda {
        fila,
        columna,
        valor,
    }
}

/// Crea una tarea para calcular una celda específica de la matriz resultante
pub fn obtener_celda_tarea(
    fila: usize,
    columna: usize,
    matriz_a: Vec<Vec<i32>>,
    matriz_b: Vec<Vec<i32>>,
) -> CeldaTarea {
    CeldaTarea {
        fila,
        columna,
        matriz_a,
        matriz_b,
    }
}

/// Valida que las matrices sean compatibles para multiplicación
pub fn validar_matrices(matriz_a: &Vec<Vec<i32>>, matriz_b: &Vec<Vec<i32>>) -> bool {
    if matriz_a.is_empty() || matriz_b.is_empty() {
        return false;
    }
    
    let _filas_a = matriz_a.len();
    let columnas_a = matriz_a[0].len();
    let filas_b = matriz_b.len();
    let columnas_b = matriz_b[0].len();
    
    // Verificar que todas las filas de A tengan la misma longitud
    for fila in matriz_a {
        if fila.len() != columnas_a {
            return false;
        }
    }
    
    // Verificar que todas las filas de B tengan la misma longitud
    for fila in matriz_b {
        if fila.len() != columnas_b {
            return false;
        }
    }
    
    // Verificar compatibilidad para multiplicación: columnas_a == filas_b
    columnas_a == filas_b
}

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
    let columnas_b = matriz_b[0].len();
    (filas_a, columnas_b)
} 
