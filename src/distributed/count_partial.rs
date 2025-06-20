// Cuenta cuántas palabras hay en un texto ya limpio
pub fn contar_palabras(texto: String, parte: usize, total: usize) -> Result<usize, String> {
    (parte < total)
        .then_some(())
        .ok_or("Part out of bounds".to_string())?;

    let subtexto = obtener_rango_particion(texto, parte, total);
    Ok(subtexto
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .count())
}

// Obtiene el slice del texto que corresponde a la parte del índice
fn obtener_rango_particion(texto: String, parte: usize, total: usize) -> String {
    let chars: Vec<char> = texto.chars().collect();
    let len = chars.len();

    let mut start = (len * parte) / total;
    let mut end = (len * (parte + 1)) / total;

    if parte > 0 && !chars[start - 1].is_whitespace() {
        while !chars[start].is_whitespace() {
            start += 1;
        }
    }

    if parte < total - 1 && !chars[end - 1].is_whitespace() {
        while !chars[end].is_whitespace() {
            end += 1;
        }
    }

    chars[start..end].iter().collect()
}
