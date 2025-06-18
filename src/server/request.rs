use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub uri: Vec<String>,
    pub params: HashMap<String, String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: HashMap<String, String>,
}

impl HttpRequest {
    pub fn new(method: String, uri: Vec<String>, params: HashMap<String, String>, version: String, headers: HashMap<String, String>, body: HashMap<String, String>) -> HttpRequest {
        HttpRequest {
            method,
            uri,
            params,
            version,
            headers,
            body,
        }
    }

    pub fn basic(method: String) -> HttpRequest {
        let uri = vec!["".to_string()];
        let params = HashMap::<String,String>::new();
        let version = "1.1".to_string();
        let headers = HashMap::<String,String>::new();
        let body = HashMap::<String,String>::new();

        Self::new(method, uri, params, version, headers, body)
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uri = format!("{:#?}", self.uri);
        let params = format!("{:#?}", self.params);
        let headers = format!("{:#?}", self.headers);
        let body = format!("{:#?}", self.body);

        write!(
            f,
            "HTTP Message\n------------\nMethod:\t\t{}\nVersion:\t{}\nURI: {}\nParams: {}\nHeaders: {}\nBody: {}\n------------",
            self.method,
            self.version,
            uri,
            params,
            headers,
            body,
        )
    }
}
