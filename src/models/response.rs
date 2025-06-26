use std::fmt;
use std::collections::HashMap;
use std::convert::From;
use std::io::{BufRead, BufReader, Read};

#[derive(Debug, Clone)]
pub enum Response {
    HTTP(HttpResponse),
    Buffer(Vec<u8>)
}

#[derive(Default, Debug, Clone)]
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
        let version = "HTTP/1.1".to_string();
        let headers = HashMap::new();
        let contents = "".to_string();

        Self::new(version, status, headers, contents)
    }

    fn reason_from_status(status: u16) -> String {
        (match status {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            405 => "Method Not Allowed",
            501 => "Not Implemented",
            505 => "HTTP Version Not Supported",
            507 => "Insufficient Storage (WebDAV; RFC 4918)",
            _ => "Internal Server Error"
        }).to_string()
    }
}

impl From<Vec<u8>> for HttpResponse {
    fn from(value: Vec<u8>) -> Self {
        let mut res = HttpResponse::default();

        let mut buf_reader = BufReader::new(value.as_slice());
        
        // First line
        let mut response_line = String::new();
        let _ = buf_reader.read_line(&mut response_line);
        let response_line: Vec<&str> = response_line.trim().split(" ").collect();
        res.version = response_line[0].to_string();
        res.status = response_line[1].parse().unwrap();
        res.reason = response_line[2].to_string();


        //Headers
        loop {
            let mut header_line = String::new();
            let _ = buf_reader.read_line(&mut header_line);

            // The headers and body are separated by an empty line (CRLF)
            if header_line.len() == 2 {
                break
            }

            // The key-value pair is delimited by a colon char
            let header: Vec<&str> = header_line.split(":").collect();
            res.headers.insert(header[0].to_string(), header[1].trim().to_string());
        }

        //Body
        let Some(content_length) = res.headers.get("Content-Length") else {
            return res;
        };

        let content_length = content_length.parse::<u64>().unwrap();

        // If the content is 0-length'd, we can stop the function
        if content_length == 0 {
            return res;
        }

        // We read the indicated amount of bytes from the stream
        let mut content: Vec<u8> = vec![0u8; content_length.try_into().unwrap()];
        let _ = buf_reader.read_exact(&mut content);
        res.contents = String::from_utf8(content).unwrap();

        res 
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
