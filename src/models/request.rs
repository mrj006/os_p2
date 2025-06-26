use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Body {
    JSON(String),
    URLdec(()),
}

impl Default for Body {
    fn default() -> Self {
        Body::URLdec(())
    }   
}

#[derive(Default, Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub uri: Vec<String>,
    pub params: HashMap<String, String>,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Body,
}

impl HttpRequest {
    pub fn new(method: String, uri: Vec<String>, params: HashMap<String, String>, version: String, headers: HashMap<String, String>, body: Body) -> HttpRequest {
        HttpRequest {
            method,
            uri,
            params,
            version,
            headers,
            body,
        }
    }
}

// We unparse the data structure into a HTTP-formatted string
impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut uri = "/".to_string();

        // Parsed messages will have a '/' only if the resources is root
        if self.uri[0] != "/" {
            for section in &self.uri {
                uri += section;
                uri += "/";
            }
    
            // We remove the extra '/' added by the for-loop
            uri.pop();
        }

        let mut params = String::new();

        if self.params.len() > 0 {
            params += "?";
            
            for (k,v) in &self.params {
                params += k;
                params += "=";
                params += v;
                params += "&";
            }

            // We remove the extra '&' added by the for-loop
            params.pop();
        }

        let mut headers = String::new();

        if self.headers.len() > 0 {
            for (k,v) in &self.headers {
                headers += k;
                headers += ": ";
                headers += v;
                headers += "\r\n";
            }
        }

        let body = match &self.body {
            Body::JSON(json) => json.to_string(),
            Body::URLdec(_) => "".to_string(),
        };

        write!(
            f,
            "{} {}{} {}\r\n{}\r\n{}",
            self.method,
            uri,
            params,
            self.version,
            headers,
            body
        )
    }
}
