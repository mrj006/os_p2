use std::fs::File;
use std::io::Write;

pub fn createfile(name: &str, content: &str, repeat: usize) -> std::io::Result<()> {
    let mut file = File::create(name)?;
    for _ in 0..repeat {
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
