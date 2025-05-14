use std::fs;

pub fn deletefile(name: &str) -> std::io::Result<()> {
    fs::remove_file(name)
}
