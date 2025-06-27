use parking_lot::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::select;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::models::matrix;
use crate::models::status::Status;
use crate::{errors, functions};
use crate::models::request::{Body, HttpRequest};
use crate::models::response::{HttpResponse, Response};
use crate::redis_comm;

use super::slaves;

pub async  fn handle_route(req: HttpRequest, remote: SocketAddr) -> Response {
    // Based on parsing logic, the vector will always have at least 1 item
    let base_uri = req.uri[0].as_str();

    match base_uri {
        "createfile" => send_request_atomic(req).await,
        "deletefile" => send_request_atomic(req).await,
        "fibonacci" => send_request_atomic(req).await,
        "hash" => send_request_atomic(req).await,
        "help" => help(req),
        "loadtest" => loadtest(req).await,
        "random" => send_request_atomic(req).await,
        "reverse" => send_request_atomic(req).await,
        "simulate" => send_request_atomic(req).await,
        "sleep" => send_request_atomic(req).await,
        "timestamp" => send_request_atomic(req).await,
        "toupper" => send_request_atomic(req).await,
        "countwords" => count_words(req).await,
        "matrixmult" => matrix_multiplication(req).await,
        "workers" => workers(req).await,
        "slave" => add_slave(req, remote),
        _ => Response::HTTP(HttpResponse::basic(404))
    }
}

fn help(req: HttpRequest) -> Response {
    if req.method != "GET" {
        return Response::HTTP(HttpResponse::basic(405));
    }

    let run = functions::help::help();
    Response::HTTP(valid_request(run))
}

async fn loadtest(req: HttpRequest) -> Response {
    if req.method != "GET" {
        return Response::HTTP(HttpResponse::basic(405));
    }

    let Some(tasks) = req.params.get("tasks") else {
        return Response::HTTP(invalid_request("Missing parameter: tasks".to_string()));
    };

    let Some(sleep) = req.params.get("sleep") else {
        return Response::HTTP(invalid_request("Missing parameter: sleep".to_string()));
    };

    let Ok(tasks) = tasks.parse::<u64>() else {
        return Response::HTTP(invalid_request("Unable to parse tasks!".to_string()));
    };

    let Ok(sleep) = sleep.parse::<u64>() else {
        return Response::HTTP(invalid_request("Unable to parse sleep!".to_string()));
    };

    for _ in 0..tasks {
        let method = "GET".to_string();
        let uri = vec!["sleep".to_string()];
        let mut params = HashMap::new();
        params.insert("seconds".to_string(), sleep.to_string());
        let version = "HTTP/1.1".to_string();
        let headers = HashMap::new();
        let body = Body::JSON(String::new());
    
        let req = HttpRequest::new(method, uri, params, version, headers, body);
        send_request_atomic(req).await;
    }

    let contents = format!("{tasks} sleep tasks with a duration of {sleep} seconds were spawned");
    Response::HTTP(valid_request(contents))
}

async fn count_words(req: HttpRequest) -> Response {
    // We handle scenarios in the master to simplify slaves' execution of this
    // parallelized job
    if req.method != "GET" {
        return Response::HTTP(HttpResponse::basic(405));
    }

    let Some(name) = req.params.get("name") else {
        return Response::HTTP(invalid_request("Missing parameter: name".to_string()));
    };

    let filepath = format!("archivos/{}", name);

    match std::fs::exists(filepath) {
        Ok(res) => {
            if !res {
                return Response::HTTP(invalid_request("Could not read file".to_string()));
            }
        },
        Err(_) => return Response::HTTP(invalid_request("Could not read file".to_string())),
    };

    let parts = slaves::get_quantity();

    if !(parts > 0) {
        return Response::HTTP(missing_slaves());
    }

    // This set allocates all partial tasks handles so we can check for errors
    let mut partial_task_handles = JoinSet::<()>::new();

    // We create al partial http request
    for i in 0..parts {
        let mut partial = HttpRequest::default();
        partial.method = req.method.clone();
        partial.uri.push("countpartial".to_string());
        partial.params.insert("name".to_string(), name.to_string());
        partial.params.insert("part".to_string(), i.to_string());
        partial.params.insert("total".to_string(), parts.to_string());
        partial.version = req.version.clone();
        partial.headers = req.headers.clone();

        // We unwrap the error to allow the task to panic. That way, we can
        // error put of the join and cancel all remaining tasks
        partial_task_handles.spawn(async move {
                let is_done = Arc::new(Mutex::new(false));
            
                // The loop will automatically break when either the task resolves
                // or there are no more slaves available
                while !send_request_partial(partial.clone(), Arc::clone(&is_done)).await.unwrap() {}

                let mut is_done = is_done.lock();
                *is_done = true;
        });
    }
    
    while let Some(res) = partial_task_handles.join_next().await {
        // The only error possible is being out of slaves
        if let Err(_) = res {
            partial_task_handles.abort_all();
            return Response::HTTP(missing_slaves());
        }
    }

    // At this point, we send the request to aggregate results
    let mut aggregate = HttpRequest::default();
    aggregate.method = req.method.clone();
    aggregate.uri.push("counttotal".to_string());
    aggregate.params.insert("name".to_string(), name.to_string());
    aggregate.version = req.version.clone();
    aggregate.headers = req.headers.clone();

    send_request_atomic(aggregate).await
}

async fn matrix_multiplication(req: HttpRequest) -> Response {
    // We handle scenarios in the master to simplify slaves' execution of this
    // parallelized job
    if req.method != "GET" {
        return Response::HTTP(HttpResponse::basic(405));
    }

    let body = match req.body {
        Body::JSON(content) => content,
        _ => return Response::HTTP(invalid_request("Missing JSON content with matrices!".to_string())),
    };

    // We parse the JSON using the matrix struct as the required data structure
    // If the JSON doesn't adhere to the struct, we can error out
    let Ok(matrices) = serde_json::from_str::<matrix::MatrixMultInput>(&body) else {
        return Response::HTTP(invalid_request("Invalid JSON format for matrices!".to_string()));
    };

    if let Err(e) = crate::distributed::matrix_total::validate_matrices(&matrices) {
        return Response::HTTP(invalid_request(e.to_string()));
    };

    // We use this ID as part of the redis key
    let job = Uuid::new_v4().to_string();
    // We save the matrices on redis to simplify HTTP message to slaves
    if let Err(_) = redis_comm::matrix_store::add_matrices_input(&job, &matrices) {
        return Response::HTTP(server_issue_response());
    }

    let rows = matrices.matrix_a.matrix.len();
    let columns = matrices.matrix_b.matrix[0].len();

    // This set allocates all partial tasks handles so we can check for errors
    let mut partial_task_handles = JoinSet::<()>::new();

    // We create al partial http request
    for i in 0..rows {
        for j in 0..columns {
            let mut partial = HttpRequest::default();
            partial.method = req.method.clone();
            partial.uri.push("matrixpartial".to_string());
            partial.params.insert("job".to_string(), job.clone());
            partial.params.insert("row".to_string(), i.to_string());
            partial.params.insert("column".to_string(), j.to_string());
            partial.version = req.version.clone();
            partial.headers = req.headers.clone();

            // We unwrap the error to allow the task to panic. That way, we can
            // error put of the join and cancel all remaining tasks
            partial_task_handles.spawn(async move {
                let is_done = Arc::new(Mutex::new(false));
            
                // The loop will automatically break when either the task resolves
                // or there are no more slaves available
                while !send_request_partial(partial.clone(), Arc::clone(&is_done)).await.unwrap() {}

                let mut is_done = is_done.lock();
                *is_done = true;
            });
        }
    }

    while let Some(res) = partial_task_handles.join_next().await {
        // The only error possible is being out of slaves
        if let Err(_) = res {
            partial_task_handles.abort_all();
            return Response::HTTP(missing_slaves());
        }
    }

    let Ok(matrix) = redis_comm::matrix_store::get_matrix_res(&job) else {
        return Response::HTTP(server_issue_response());
    };

    // At this point, we send the request to aggregate results
    let mut aggregate = HttpRequest::default();
    aggregate.method = req.method.clone();
    aggregate.uri.push("matrixtotal".to_string());
    aggregate.params.insert("job".to_string(), job);
    aggregate.version = req.version.clone();
    aggregate.headers = req.headers.clone();
    aggregate.body = Body::JSON(serde_json::to_string(&matrix).unwrap());

    send_request_atomic(aggregate).await
}

async fn workers(req: HttpRequest) -> Response {
    // We handle scenarios in the master to simplify slaves' execution of this
    // parallelized job
    if req.method != "GET" {
        return Response::HTTP(HttpResponse::basic(405));
    }

    let slaves = slaves::get_quantity();

    let mut worker_status: Vec<Status> = vec![];

    // This set allocates all partial tasks handles so we can check for errors
    let mut partial_task_handles: JoinSet<Response> = (0..slaves)
        .map(|index| {
            let mut req = req.clone();
            req.uri = vec!["status".to_string()];
            
            async move {
                let (slave, token) = slaves::get_specific(index).unwrap();
                send_request_specific(req, slave, token).await
            }
        }).collect();

    while let Some(res) = partial_task_handles.join_next().await {
        // We can ignore the error because it means the slave is gone
        if let Ok(res) = res {
            // The function only returns buffers, so we can ignore the http arm
            if let Response::Buffer(buf) = res {
                let res = HttpResponse::from(buf);
                let status = serde_json::from_str::<Status>(&res.contents).unwrap();
                worker_status.push(status);
            }
        }
    }

    if worker_status.len() == 0 {
        return Response::HTTP(missing_slaves());
    }

    let version = "HTTP/1.1".to_string();
    let status = 200;
    let headers = HashMap::new();
    let contents = serde_json::to_string(&worker_status).unwrap();

    Response::HTTP(HttpResponse::new(version, status, headers, contents))
}

fn invalid_request(contents: String) -> HttpResponse {
    let version = "HTTP/1.1".to_string();
    let status = 400;
    let headers = HashMap::new();
    
    HttpResponse::new(version, status, headers, contents)
}

fn missing_slaves() -> HttpResponse {
    let version = "HTTP/1.1".to_string();
    let status = 500;
    let headers = HashMap::new();
    let contents = "Unable to process your request at this time.\nTry again later.".to_string();
    HttpResponse::new(version, status, headers, contents)
}

fn valid_request(contents: String) -> HttpResponse {
    let version = "HTTP/1.1".to_string();
    let status = 200;
    let headers = HashMap::new();
    
    HttpResponse::new(version, status, headers, contents)
}

fn server_issue_response() -> HttpResponse {
    let contents = "Unable to process your request at this time!".to_string();
    HttpResponse::new("HTTP/1.1".to_string(), 500, HashMap::new(), contents)
}

async fn send_request_atomic(req: HttpRequest) -> Response {
    let is_done = Arc::new(Mutex::new(false));

    loop {
        let res = send_request_base(req.clone(), Arc::clone(&is_done)).await.unwrap(); 
        let mut is_done = is_done.lock();
        *is_done = true;

        return res;
    }
}

async fn send_request_specific(req: HttpRequest, slave: SocketAddr, token: CancellationToken) -> Response {
    let is_done = Arc::new(Mutex::new(false));

    loop {
        let done_clone = Arc::clone(&is_done);
        let token_clone = token.clone();

        tokio::spawn(async move {
            loop {
                let token_clone = token.clone();
                monitor_slave(slave, token_clone).await;

                let is_done = done_clone.lock();

                if *is_done {
                    break;
                }
            }
        });

        let res = select! {
            buffer = send_request_slave(slave, req.clone()) => {
                Ok(Response::Buffer(buffer))
            }

            _ = token_clone.cancelled() => {
                Err(Box::new(errors::slaves::SlaveFailedError))
            }
        };

        let res = res.unwrap();
        let mut is_done = is_done.lock();
        *is_done = true;

        return res;
    }
}

async fn send_request_base(req: HttpRequest, is_done: Arc<Mutex<bool>>) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
    let done_clone = Arc::clone(&is_done);

    let Some((socket, token)) = slaves::get_current() else {
        return Ok(Response::HTTP(missing_slaves()));
    };

    let token_clone = token.clone();
    tokio::spawn(async move {
        loop {
            let token_clone = token.clone();
            monitor_slave(socket, token_clone).await;

            let is_done = done_clone.lock();

            if *is_done {
                break;
            }
        }
    });

    select! {
        buffer = send_request_slave(socket, req) => {
            Ok(Response::Buffer(buffer))
        }

        _ = token_clone.cancelled() => {
            Err(Box::new(errors::slaves::SlaveFailedError))
        }
    }
}

// We error out if there are no available slaves for assignment
async fn send_request_partial(req: HttpRequest, is_done: Arc<Mutex<bool>>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let done_clone = Arc::clone(&is_done);

    let Some((socket, token)) = slaves::get_current() else {
        let mut is_done = is_done.lock();
        *is_done = true;
        
        return Err(Box::new(errors::slaves::SlavesMissingError));
    };

    let token_clone = token.clone();

    tokio::spawn(async move {
        loop {
            let token_clone = token.clone();
            monitor_slave(socket, token_clone).await;

            let is_done = done_clone.lock();

            if *is_done {
                break;
            }
        }
    });

    select! {
        _ = send_request_slave(socket, req) => {
            Ok(true)
        }

        _ = token_clone.cancelled() => {
            Ok(false)
        }
    }
}

async fn send_request_slave(socket: SocketAddr, req: HttpRequest) -> Vec<u8> {
    let message = format!("{}", req);

    let mut stream = TcpStream::connect(socket).await.unwrap();
    let _ = stream.write_all(message.as_bytes()).await;
    
    loop {
        // Wait for the socket to be readable
        let _ = stream.readable().await;

        let mut buf = vec![0; 4096];

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read(&mut buf) {
            Ok(0) => return vec![0; 0],
            Ok(n) => {
                buf.truncate(n);
                return buf;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(_) => {
                return vec![0; 0];
            }
        }
    }
}

async fn monitor_slave(socket: SocketAddr, token: CancellationToken) -> bool {
    let mut ping = HttpRequest::default();
    ping.method = "GET".to_string();
    ping.uri.push("ping".to_string());
    ping.version = "HTTP/1.1".to_string();

    select! {
        _ = send_request_slave(socket, ping) => {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            true
        }
        
        // If this branch completes, we need to cancel all slave-related tasks
        // and remove it from the map to avoid further assignments
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            token.cancel();            
            let _ = slaves::remove(socket);
            false
        }
    }
}

fn add_slave(req: HttpRequest, remote: SocketAddr) -> Response {
    let slave_code = env::var("SLAVE_CODE").unwrap();

    let Some(port) = req.params.get("port") else {
        return Response::HTTP(invalid_request("Missing port parameter!".to_string()));
    };

    let Ok(port) = port.parse::<u16>() else {
        return Response::HTTP(invalid_request("Invalid port parameter!".to_string()));
    };

    let Some(code) = req.params.get("slave_code") else {
        return Response::HTTP(invalid_request("Missing code parameter!".to_string()));
    };

    if slave_code != *code {
        return Response::HTTP(invalid_request("Invalid code parameter!".to_string()));
    }

    let ip_socket: SocketAddr = format!("{}:{}", remote.ip(), port).parse().unwrap();

    slaves::add(ip_socket);

    Response::HTTP(valid_request("".to_string()))
}
