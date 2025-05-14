use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub version: String,
    pub status: u16,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub contents: String,
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut headers = "".to_string();

        for (header, value) in &self.headers {
            headers = headers + &header + ": " + &value + "\r\n";
        }

        write!(f, "{} {} {}\r\n{}\r\n{}",
            self.version,
            self.status,
            self.reason,
            headers,
            self.contents,
        )
    }
}
