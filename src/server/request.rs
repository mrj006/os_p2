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
