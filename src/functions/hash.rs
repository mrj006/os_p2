use sha256::digest;

pub fn hash(text: &str) -> String {
    format!("{}", digest(text))
}
