pub mod count_partial;
pub mod count_total;
pub mod matrix_partial;
pub mod matrix_total;

pub use count_partial::{contar_palabras, obtener_rango_particion};
pub use count_total::unir_resultados;
pub use matrix_total::{calcular_dimensiones_resultado, construir_matriz_resultado, matriz_a_json};
