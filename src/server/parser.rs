use serde_json::Value;
use std::{
    collections::HashMap, io::{prelude::*, BufReader}, net::TcpStream
};

use crate::models::request::{Body, HttpRequest};
use crate::errors::*;

// This function parses a valid http request message and returns a struct or
// throws different errors to the caller
pub fn parse(mut message: &TcpStream) -> Result<HttpRequest, Box<dyn std::error::Error>> {
    let mut buf_reader = BufReader::new(&mut message);

    let (method, uri, params, version) = parse_request_line(&mut buf_reader)?;
    let headers = parse_headers(&mut buf_reader);
    let body = parse_body(&mut buf_reader, &headers)?;

    Ok(HttpRequest::new(method, uri, params, version, headers, body))
}

// This function parses the 1st line of the http message,
// returning the method, version, URI and query parameters used.
fn parse_request_line(buf_reader: &mut BufReader<&mut &TcpStream>)
    -> Result<(String, Vec<String>, HashMap<String, String>, String), parse::ParseUriError> {
    
    let mut request_line = String::new();
    let _ = buf_reader.read_line(&mut request_line);

    // The first-line values are space-separated
    let request_line: Vec<&str> = request_line.trim().split(" ").collect();

    // The URI and query params are delimited by the ? char
    let uri_line: Vec<&str> = request_line[1].split("?").collect();

    // The resulting vector can have a length of 1..2
    // Thus, throwing an error back to the caller
    if !(uri_line.len() < 3 && uri_line.len() > 0) {
        return Err(parse::ParseUriError);
    }

    // The 1st value is the actual URI
    let unchecked_uri: Vec<&str> = uri_line[0].split("/").collect();
    
    // Given the segments are separated by a slash, we add root to the vector
    let mut uri: Vec<String> = vec![];

    let mut len_to_check = unchecked_uri.len()-1;

    // If the last string in the vector is not empty (meaning a trailing slash),
    // Then we increment the variable to include it in the final URI
    if unchecked_uri[len_to_check] != "" {
        len_to_check += 1;
    }

    // If there are empty strings in the vector, it means there were multiple
    // consecutive slashes. Thus, it's an invalid URI ad we return early with
    // an error. Else, we add the string to the vector
    for i in 1..(len_to_check) {
        if unchecked_uri[i] == "" {
            return Err(parse::ParseUriError);
        }

        uri.push(unchecked_uri[i].to_string());
    }

    // If the vector is empty, we know the request is for the root URI
    // This guarantees the vector's size is at least 1
    if uri.len() == 0 {
        uri.push("/".to_string());
    }

    let method = request_line[0].to_string();
    let version = request_line[2].to_string();
    let params = if uri_line.len() > 1 {
        parse_urlencoded(uri_line[1].to_string())
    } else {
        HashMap::new()
    };

    Ok((method, uri, params, version))
}

fn parse_headers(buf_reader: &mut BufReader<&mut &TcpStream>) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();

    // Each header is separated by a new line (CRLF)
    loop {
        let mut line = String::new();
        let _ = buf_reader.read_line(&mut line);

        // The headers and body are separated by an empty line (CRLF)
        if line.len() == 2 {
            break
        }

        // The key-value pair is delimited by a colon char
        let header: Vec<&str> = line.split(":").collect();
        headers.insert(header[0].to_string(), header[1].trim().to_string());
    }

    headers
}

fn parse_body(buf_reader: &mut BufReader<&mut &TcpStream>, headers: &HashMap<String, String>) -> Result<Body, Box<dyn std::error::Error>> {
    let mut body = Body::JSON(String::new());

    // If the header is missing, we return early with an empty hashmap
    let Some(content_length) = headers.get("Content-Length") else {
        return Ok(body);
    };

    let content_length = content_length.parse::<u64>()?;

    // If the content is 0-length'd, we can stop the function
    if content_length == 0 {
        return Ok(body);
    }

    let content_type = headers.get("Content-Type");

    // This implies the request has a body but it didn't report the type
    // thus, it's invalid
    if content_type.is_none() {
        return Err(Box::new(parse::ParseUriError));
    }
    
    // This unwrap shouldn't fail as we checked it and return early in the if above
    let content_type = content_type.unwrap().to_string();

    if content_type != "application/x-www-form-urlencoded" && content_type != "application/json" {
        return Err(Box::new(implement::ImplementationError));
    }
    
    // We read the indicated amount of bytes from the stream
    let mut content: Vec<u8> = vec![];
    let mut buffer_to_read = buf_reader.take(content_length);
    let _ = buffer_to_read.read_to_end(&mut content);
    let content = String::from_utf8(content)?;

    if content_type == "application/json" {
        // We don't attempt to parse it, as we only need to check whether the
        // JSON is mal-formed or not
        if let Err(_) = serde_json::from_str::<Value>(&content) {
            return Err(Box::new(parse::ParseUriError));
        }

        body = Body::JSON(content);
    } else {
        body = Body::URLdec(());        
    }

    Ok(body)
}

// Parser would work for query params and body as x-www-form-urlencoded
fn parse_urlencoded(content: String) -> HashMap<String, String> {
    let mut parsed: HashMap<String, String> = HashMap::new();

    // This encoding has key-value pairs joined by &
    for pair in content.split("&") {
        // The key-value pair is delimited by an equal char
        let pair: Vec<&str> = pair.split("=").collect();
        parsed.insert(pair[0].to_string(), pair[1].to_string());
    }

    parsed
}
