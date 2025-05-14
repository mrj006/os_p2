use std::fs;
use std::io::Write;

pub fn createfile(name: &str, content: &str, repeat: u64) -> std::io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(name)?;


    for _ in 0..repeat {
        write!(file, "{}", content)?;
    }
    
    Ok(())
}
