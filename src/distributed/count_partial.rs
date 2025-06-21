pub fn count_part_words(text: String, part: usize, total: usize) -> Result<usize, String> {
    (part < total)
        .then_some(())
        .ok_or("Part out of bounds".to_string())?;

    let subtext = get_part_range(text, part, total);

    Ok(subtext
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .count())
}

// We get a sub string based on the part specified
fn get_part_range(text: String, part: usize, total: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    let mut start = (len * part) / total;
    let mut end = (len * (part + 1)) / total;

    if part > 0 && !chars[start - 1].is_whitespace() {
        while !chars[start].is_whitespace() {
            start += 1;
        }
    }

    if part < total - 1 && !chars[end - 1].is_whitespace() {
        while !chars[end].is_whitespace() {
            end += 1;
        }
    }

    chars[start..end].iter().collect()
}
