use std::{collections::HashMap, io::Write, net::{SocketAddr, TcpStream}};

use super::{request::HttpRequest, response::HttpResponse};
use crate::functions;

pub fn handle_route(req: HttpRequest, address: SocketAddr) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Basedon parsing logic, the vecotr will always have at least 1 item
    let base_uri = req.uri[0].as_str();

    match base_uri {
        "createfile" => createfile(req),
        "deletefile" => deletefile(req),
        "fibonacci" => Ok(fibonacci(req)),
        "hash" => Ok(hash(req)),
        "help" => Ok(help(req)),
        "loadtest" => loadtest(req, address),
        "random" => Ok(random(req)),
        "reverse" => Ok(reverse(req)),
        "simulate" => Ok(simulate(req)),
        "sleep" => Ok(sleep(req)),
        "timestamp" => Ok(timestamp(req)),
        "toupper" => Ok(toupper(req)),
        _ => Ok(HttpResponse::basic(404))
    }
}

fn createfile(req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if req.method != "POST" {
        return Ok(HttpResponse::basic(405));
    }

    let name = req.params.get("name");
    let content = req.params.get("content");
    let repeat = req.params.get("repeat");

    if !(name.is_some() && content.is_some() && repeat.is_some()) {
        return Ok(invalid_request("Invalid query params provided!".to_string()));
    }

    let name = name.unwrap();
    let content = content.unwrap();
    let repeat = repeat.unwrap().parse::<u64>();

    if let Err(_) = repeat {
        return Ok(invalid_request("Unable to parse repeat param!".to_string()));
    }

    let repeat = repeat.unwrap();
    let run = functions::createfile::createfile(name, content, repeat);

    if let Err(_) = run {
        return Ok(invalid_request("File already exists!".to_string()));
    }

    Ok(HttpResponse::basic(200))
}

fn deletefile(req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if req.method != "DELETE" {
        return Ok(HttpResponse::basic(405));
    }

    let name = req.params.get("name");

    if name.is_none() {
        return Ok(invalid_request("Invalid query params provided!".to_string()));
    }

    let name = name.unwrap();
    let run = functions::deletefile::deletefile(name);

    if let Err(_) = run {
        return Ok(invalid_request("Unable to delete file!".to_string()));
    }

    Ok(HttpResponse::basic(200))
}

fn fibonacci(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let num = req.params.get("num");

    if num.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let num = num.unwrap().parse::<u128>();
    
    if let Err(_) = num {
        return invalid_request("Unable to parse num param!".to_string());
    }

    let num = num.unwrap();
    let run = functions::fibonacci::fibonacci(num);
    
    if run.is_none() {
        return HttpResponse::basic(507);
    }

    valid_request(run.unwrap().to_string())
}

fn hash(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let text = req.params.get("text");

    if text.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let text = text.unwrap();
    let run = functions::hash::hash(text);
    valid_request(run)
}

fn help(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let run = functions::help::help();
    valid_request(run)
}

fn loadtest(req: HttpRequest, address: SocketAddr) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    if req.method != "GET" {
        return Ok(HttpResponse::basic(405));
    }

    let tasks = req.params.get("tasks");
    let sleep = req.params.get("sleep");

    if !(tasks.is_some() && sleep.is_some()) {
        return Ok(invalid_request("Invalid query params provided!".to_string()));
    }

    let tasks = tasks.unwrap().parse::<u64>();
    let sleep = sleep.unwrap().parse::<u64>();

    if let Err(_) = tasks {
        return Ok(invalid_request("Unable to parse tasks!".to_string()));
    }

    if let Err(_) = sleep {
        return Ok(invalid_request("Unable to parse sleep!".to_string()));
    }

    let tasks = tasks.unwrap();
    let sleep = sleep.unwrap();

    let request = format!("GET /sleep?seconds={sleep} HTTP/1.1 \r\n\r\n");
    
    for _ in 0..tasks {
        let mut stream = TcpStream::connect(address)?;
        let _ = stream.write_all(request.as_bytes())?;
    }

    let contents = format!("{tasks} sleep tasks with a duration of {sleep} seconds were spawned");
    Ok(valid_request(contents))
}

fn random(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let count = req.params.get("count");
    let min = req.params.get("min");
    let max = req.params.get("max");

    if !(count.is_some() && min.is_some() && max.is_some()) {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let count = count.unwrap().parse::<usize>();
    let min = min.unwrap().parse::<i32>();
    let max = max.unwrap().parse::<i32>();

    if let Err(_) = count {
        return invalid_request("Unable to parse count!".to_string());
    }

    if let Err(_) = min {
        return invalid_request("Unable to parse min!".to_string());
    }

    if let Err(_) = max {
        return invalid_request("Unable to parse max!".to_string());
    }

    let count = count.unwrap();
    let min = min.unwrap();
    let max = max.unwrap();

    let run = functions::random::random(count, min, max);
    let run = format!("{:#?}", run);
    valid_request(run)
}

fn reverse(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let text = req.params.get("text");

    if text.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let text = text.unwrap();
    let run = functions::reverse::reverse(text);
    valid_request(run)
}

fn simulate(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let task = req.params.get("task");
    let seconds = req.params.get("seconds");

    if !(task.is_some() && seconds.is_some()) {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let task = task.unwrap();
    let seconds = seconds.unwrap().parse::<u64>();

    if let Err(_) = seconds {
        return invalid_request("Unable to parse seconds param!".to_string());
    }

    let seconds = seconds.unwrap();
    let run = functions::simulate::simulate(seconds, task);
    valid_request(run)
}

fn sleep(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let seconds = req.params.get("seconds");

    if seconds.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let seconds = seconds.unwrap().parse::<u64>();

    if let Err(_) = seconds {
        return invalid_request("Unable to parse seconds param!".to_string());
    }

    let seconds = seconds.unwrap();
    let run = functions::sleep::sleep(seconds);
    valid_request(run)
}

fn timestamp(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let run = functions::timestamp::timestamp();
    valid_request(run)
}

fn toupper(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let text = req.params.get("text");

    if text.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let text = text.unwrap();
    let run = functions::toupper::toupper(text);
    valid_request(run)
}

fn invalid_request(contents: String) -> HttpResponse {
    let res = HttpResponse::new("HTTP/1.1".to_string(), 400, HashMap::new(), contents);
    res
}

fn valid_request(contents: String) -> HttpResponse {
    let res = HttpResponse::new("HTTP/1.1".to_string(), 200, HashMap::new(), contents);
    res
}
