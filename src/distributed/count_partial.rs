// Cuenta cuántas palabras hay en un texto ya limpio
pub fn contar_palabras(subtexto: &str) -> usize {
    //println!("[{}]", subtexto);
    subtexto
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .count()
}

// Obtiene el slice del texto que corresponde a la parte del índice
pub fn obtener_rango_particion(texto: &str, parte: usize, total: usize) -> String {
    let chars: Vec<char> = texto.chars().collect();
    let len = chars.len();

    let mut start = (len * parte) / total;
    let mut end = (len * (parte + 1)) / total;

    if parte > 0 && !chars[start - 1].is_whitespace() {
        while start < len && !chars[start].is_whitespace() {
            start += 1;
        }
    }

    if !chars[end - 1].is_whitespace() {
        while end < len && !chars[end].is_whitespace() {
            end += 1;
        }
    }
    else {
        end -= 1;
    }
    

    chars[start..end.min(len)].iter().collect()
}

