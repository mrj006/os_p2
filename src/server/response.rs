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

impl HttpResponse {
    pub fn new(version: String, status: u16, headers: HashMap<String, String>, contents: String) -> HttpResponse {
        let reason = Self::reason_from_status(status);

        HttpResponse { version, status, reason, headers, contents }
    }

    pub fn basic(status: u16) -> HttpResponse {
        let reason = Self::reason_from_status(status);

        HttpResponse { version: "HTTP/1.1".to_string(), status, reason, headers: HashMap::new(), contents: "".to_string() }
    }

    fn reason_from_status(status: u16) -> String {
        (match status {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            501 => "Not Implemented",
            _ => "Internal Server Error"
        }).to_string()
    }
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
