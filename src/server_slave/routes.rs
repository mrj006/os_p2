use std::collections::HashMap;
use std::net::SocketAddr;

use crate::errors::log_error;
use crate::models::matrix::MatrixPartialRes;
use crate::models::{request::HttpRequest, response::HttpResponse};
use crate::status::status;
use crate::{distributed, functions};
use crate::redis_comm;

pub fn handle_route(req: HttpRequest, _: SocketAddr) -> HttpResponse {
    // Based on parsing logic, the vector will always have at least 1 item
    let base_uri = req.uri[0].as_str();
    update_thread_status(true, base_uri.to_string());
    println!("Route: {}", base_uri);
    match base_uri {
        "createfile" => createfile(req),
        "deletefile" => deletefile(req),
        "fibonacci" => fibonacci(req),
        "hash" => hash(req),
        "random" => random(req),
        "reverse" => reverse(req),
        "simulate" => simulate(req),
        "sleep" => sleep(req),
        "status" => status(req),
        "timestamp" => timestamp(req),
        "toupper" => toupper(req),
        "countpartial" => count_partial(req),
        "counttotal" => count_total(req),
        "matrixpartial" => matrix_partial(req),
        "matrixtotal" => matrix_total(req),
        "ping" => ping(req),
        _ => HttpResponse::basic(404)
    }
}

fn ping(_: HttpRequest) -> HttpResponse {
    valid_request("".to_string()) 
}

fn createfile(req: HttpRequest) -> HttpResponse {
    if req.method != "POST" {
        return HttpResponse::basic(405);
    }

    let name = req.params.get("name");
    let content = req.params.get("content");
    let repeat = req.params.get("repeat");

    if !(name.is_some() && content.is_some() && repeat.is_some()) {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let name = name.unwrap();
    let content = content.unwrap();
    let repeat = repeat.unwrap().parse::<u64>();

    if let Err(_) = repeat {
        return invalid_request("Unable to parse repeat param!".to_string());
    }

    let repeat = repeat.unwrap();
    match functions::createfile::createfile(name, content, repeat) {
        Ok(_) => HttpResponse::basic(200),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return invalid_request("File already exists!".to_string());
            } else {
                return HttpResponse::basic(500);
            }
        }
    }
}

fn deletefile(req: HttpRequest) -> HttpResponse {
    if req.method != "DELETE" {
        return HttpResponse::basic(405);
    }

    let name = req.params.get("name");

    if name.is_none() {
        return invalid_request("Invalid query params provided!".to_string());
    }

    let name = name.unwrap();
    let run = functions::deletefile::deletefile(name);

    if let Err(_) = run {
        return invalid_request("Unable to delete file!".to_string());
    }

    HttpResponse::basic(200)
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

    match run {
        Ok(vec) => valid_request(format!("{:#?}", vec)),
        Err(msg) => invalid_request(msg),
    }
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

fn status(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let contents = status::status();

    let content_length = contents.len();
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Content-Length".to_string(), content_length.to_string());
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    HttpResponse::new("HTTP/1.1".to_string(), 200, headers, contents)
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

fn count_partial(req: HttpRequest) -> HttpResponse {
    // We can simplify slaves' checks as we perform them in the master
    // - params provided
    // - parsing of params

    let name = req.params.get("name").unwrap();
    let part = req.params.get("part").unwrap();
    let total = req.params.get("total").unwrap();
    let part_index = part.parse::<usize>().unwrap();
    let total_parts= total.parse::<usize>().unwrap();

    // We keep this check, as the file could've been manipulated by a 3rd party
    // between executions
    let filepath = format!("archivos/{}", name);
    let Ok(text) = std::fs::read_to_string(filepath) else {
        return invalid_request("Could not read file".to_string());
    };
    
    let count = distributed::count_partial::count_part_words(text, part_index, total_parts);
    
    match redis_comm::count_store::add_count_part_res(name, part, count) {
        Ok(_) => valid_request(format!("file={},part={},words={}", name, part, count)),
        Err(e) => redis_down_response(Box::new(e)),
    }
}

fn count_total(req: HttpRequest) -> HttpResponse {
    // We can simplify slaves' checks as we perform them in the master
    // - params provided
    // - parsing of params

    let name = req.params.get("name").unwrap();

    let values = match redis_comm::count_store::get_count_part_res(name) {
        Ok(values) => values,
        Err(e) => return redis_down_response(Box::new(e)),
    };

    let res = distributed::count_total::count_join(values);
    if let Err(e) = redis_comm::count_store::remove_count_part_res(&name) {
        log_error(Box::new(e));
    }

    if let Err(e) = redis_comm::count_store::add_count_res(name, res) {
        log_error(Box::new(e));
    }

    valid_request(format!("file={},total={}", name, res))
}

fn matrix_partial(req: HttpRequest) -> HttpResponse {
    // We can simplify slaves' checks as we perform them in the master
    // - params provided
    // - parsing of params

    let job = req.params.get("job").unwrap();
    let row = req.params.get("row").unwrap();
    let column = req.params.get("column").unwrap();
    let row = row.parse::<usize>().unwrap();
    let column = column.parse::<usize>().unwrap();
    
    let matrices = match redis_comm::matrix_store::get_matrices_input(job) {
        Ok(matrices) => matrices,
        Err(e) => return redis_down_response(Box::new(e)),
    };

    let value =  distributed::matrix_partial::matrix_cell_value(matrices, row, column);
    let cell = MatrixPartialRes { row, column, value };

    if let Err(e) = redis_comm::matrix_store::add_matrix_part_res(job, cell) {
        return redis_down_response(Box::new(e));
    }

    valid_request(format!("row={}, column={}, value={}", row, column, value))
}

fn matrix_total(req: HttpRequest) -> HttpResponse {
    // We can simplify slaves' checks as we perform them in the master
    // - params provided
    // - parsing of params

    let job = req.params.get("job").unwrap();

    let matrices = match redis_comm::matrix_store::get_matrices_input(job) {
        Ok(matrices) => matrices,
        Err(e) => return redis_down_response(Box::new(e)),
    };

    let values = match redis_comm::matrix_store::get_all_matrix_part_res(job) {
        Ok(values) => values,
        Err(e) => return redis_down_response(e),
    };

    let rows = matrices.matrix_a.matrix.len();
    let columns = matrices.matrix_b.matrix[0].len();

    let res = distributed::matrix_total::matrix_multi_join(rows, columns, values);
    
    if let Err(e) = redis_comm::matrix_store::add_matrix_res(job, &res) {
        return redis_down_response(Box::new(e));
    }

    let res = serde_json::to_string(&res).unwrap();
    let _ = redis_comm::matrix_store::remove_job(job);

    let version = "HTTP/1.1".to_string();
    let status = 200;
    let mut headers = HashMap::<String, String>::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Content-Length".to_string(), res.len().to_string());

    HttpResponse::new(version, status, headers, res)
}

fn invalid_request(contents: String) -> HttpResponse {
    update_thread_status(false, "".to_string());
    HttpResponse::new("HTTP/1.1".to_string(), 400, HashMap::new(), contents)
}

fn valid_request(contents: String) -> HttpResponse {
    update_thread_status(false, "".to_string());
    HttpResponse::new("HTTP/1.1".to_string(), 200, HashMap::new(), contents)
}

fn redis_down_response(error: Box<dyn std::error::Error>) -> HttpResponse {
    update_thread_status(false, "".to_string());
    let contents = (&error).to_string();
    log_error(error);
    HttpResponse::new("HTTP/1.1".to_string(), 500, HashMap::new(), contents)
}

fn update_thread_status(busy: bool, command: String) {
    let pid = gettid::gettid();
    status::update_thread(pid, busy, command);

    if busy {
        status::increase_requests_handled();
    }
}
