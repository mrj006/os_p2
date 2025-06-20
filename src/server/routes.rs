use std::{collections::HashMap, io::{Read, Write}, net::{SocketAddr, TcpStream}};

use super::{request::HttpRequest, response::HttpResponse};
use crate::{functions, status::status};
use crate::{distributed::{count_partial, count_total, matrix_partial, matrix_total}};

pub fn handle_route(req: HttpRequest, port: u16) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    // Based on parsing logic, the vector will always have at least 1 item
    let base_uri = req.uri[0].as_str();
    let pid = gettid::gettid();
    status::update_worker(pid, true, base_uri.to_string());
    status::increase_requests_handled();

    match base_uri {
        "createfile" => createfile(req),
        "deletefile" => deletefile(req),
        "fibonacci" => Ok(fibonacci(req)),
        "hash" => Ok(hash(req)),
        "help" => Ok(help(req)),
        "loadtest" => loadtest(req, port),
        "random" => Ok(random(req)),
        "reverse" => Ok(reverse(req)),
        "simulate" => Ok(simulate(req)),
        "sleep" => Ok(sleep(req)),
        "status" => Ok(status(req)),
        "timestamp" => Ok(timestamp(req)),
        "toupper" => Ok(toupper(req)),
        "countpartial" => Ok(count_partial(req)),
        "counttotal" => Ok(count_total(req)),
        "matrixpartial" => Ok(matrix_partial(req)),
        "matrixtotal" => Ok(matrix_total(req)),
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
    match functions::createfile::createfile(name, content, repeat) {
        Ok(_) => Ok(HttpResponse::basic(200)),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Ok(invalid_request("File already exists!".to_string()));
            } else {
                return Ok(HttpResponse::basic(500));
            }
        }
    }
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

fn loadtest(req: HttpRequest, port: u16) -> Result<HttpResponse, Box<dyn std::error::Error>> {
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
    
    for n in 0..tasks {
        let mut stream = TcpStream::connect(SocketAddr::from(([127, 0, 0, 1], port)))?;
        let _ = stream.write_all(request.as_bytes())?;

        if n == (tasks - 1) {
            let mut response = String::new();
            let _ = stream.read_to_string(&mut response);
        }
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
    // Validamos método HTTP
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    // Extraemos parámetros de la URL
    let Some(name) = req.params.get("name") else {
        return invalid_request("Missing parameter: name".to_string());
    };
    let Some(part) = req.params.get("part") else {
        return invalid_request("Missing parameter: part".to_string());
    };
    let Some(total) = req.params.get("total") else {
        return invalid_request("Missing parameter: total".to_string());
    };

    // Convertimos parámetros a números
    let Ok(part_index) = part.parse::<usize>() else {
        return invalid_request("Invalid value for 'part'".to_string());
    };
    let Ok(total_parts) = total.parse::<usize>() else {
        return invalid_request("Invalid value for 'total'".to_string());
    };

    // Armamos ruta al archivo y leemos el contenido
    let filepath = format!("archivos/{}", name);
    let Ok(contenido) = std::fs::read_to_string(&filepath) else {
        return invalid_request("Could not read file".to_string());
    };

    // Extraemos el subtexto correspondiente y contamos palabras
    let sub = count_partial::obtener_rango_particion(&contenido, part_index, total_parts);
    let count = count_partial::contar_palabras(&sub);

    // Armamos respuesta como string
    let resultado = format!("archivo={},palabras={}", name, count);
    valid_request(resultado)
}

fn count_total(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    let Some(name) = req.params.get("name") else {
        return invalid_request("Missing parameter: name".to_string());
    };
    let Some(results_str) = req.params.get("results") else {
        return invalid_request("Missing parameter: results".to_string());
    };

    // Normalizar el parámetro results para evitar problemas de saltos de línea o espacios
    let results_str = results_str.replace("\n", "").replace("\r", "").replace(" ", "");
    println!("results_str normalizado: {:?}", results_str);
    // Convertimos string "3,4,2" en vector [3,4,2]
    let resultados: Option<Vec<usize>> = results_str
        .split(',')
        .map(|s| s.trim().parse::<usize>().ok())
        .collect();

    let Some(valores) = resultados else {
        return invalid_request("Invalid format for 'results'".to_string());
    };

    let suma = count_total::unir_resultados(&valores);
    let respuesta = format!("archivo={},total_palabras={}", name, suma);
    valid_request(respuesta)
}

fn matrix_partial(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    // Extraer parámetros
    let Some(fila_str) = req.params.get("fila") else {
        return invalid_request("Missing parameter: fila".to_string());
    };
    let Some(columna_str) = req.params.get("columna") else {
        return invalid_request("Missing parameter: columna".to_string());
    };
    let Some(matriz_a_str) = req.params.get("matriz_a") else {
        return invalid_request("Missing parameter: matriz_a".to_string());
    };
    let Some(matriz_b_str) = req.params.get("matriz_b") else {
        return invalid_request("Missing parameter: matriz_b".to_string());
    };

    // Parsear parámetros numéricos
    let Ok(fila) = fila_str.parse::<usize>() else {
        return invalid_request("Invalid value for 'fila'".to_string());
    };
    let Ok(columna) = columna_str.parse::<usize>() else {
        return invalid_request("Invalid value for 'columna'".to_string());
    };

    // Parsear matrices JSON
    let Ok((matriz_a, matriz_b)) = matrix_partial::parse_matrices(matriz_a_str, matriz_b_str) else {
        return invalid_request("Invalid JSON format for matrices".to_string());
    };

    // Validar matrices
    if !matrix_partial::validar_matrices(&matriz_a, &matriz_b) {
        return invalid_request("Matrices are not compatible for multiplication".to_string());
    }

    // Crear tarea y calcular celda
    let tarea = matrix_partial::obtener_celda_tarea(fila, columna, matriz_a, matriz_b);
    let resultado = matrix_partial::calcular_celda_matriz(&tarea);

    // Formatear respuesta
    let respuesta = format!("fila={},columna={},valor={}", resultado.fila, resultado.columna, resultado.valor);
    valid_request(respuesta)
}

fn matrix_total(req: HttpRequest) -> HttpResponse {
    if req.method != "GET" {
        return HttpResponse::basic(405);
    }

    // Extraer parámetros
    let matriz_a_str = match req.params.get("matriz_a") {
        Some(s) => s,
        None => return invalid_request("Missing parameter: matriz_a".to_string()),
    };
    let matriz_b_str = match req.params.get("matriz_b") {
        Some(s) => s,
        None => return invalid_request("Missing parameter: matriz_b".to_string()),
    };
    let results_str = match req.params.get("results") {
        Some(s) => s,
        None => return invalid_request("Missing parameter: results".to_string()),
    };
    
    // Parsear las matrices
    let (matriz_a, matriz_b) = match matrix_partial::parse_matrices(matriz_a_str, matriz_b_str) {
        Ok(matrices) => matrices,
        Err(e) => return invalid_request(format!("Invalid JSON for matrices: {}", e)),
    };
    
    // Validar compatibilidad de matrices
    if !matrix_partial::validar_matrices(&matriz_a, &matriz_b) {
        return invalid_request("Matrices are not compatible for multiplication".to_string());
    }

    // Normalizar el parámetro results para evitar problemas de saltos de línea o espacios
    let results_str = results_str.replace("\n", "").replace("\r", "").replace(" ", "");
    println!("results_str normalizado: {:?}", results_str);
    // Intentar parsear los resultados directamente
    let mut resultados: Vec<matrix_partial::ResultadoCelda> = results_str
        .split(';')
        .filter_map(|celda_str| {
            if celda_str.is_empty() { return None; }
            let mut fila = 0;
            let mut columna = 0;
            let mut valor = 0;
            let mut parts_count = 0;
            for parte in celda_str.split(',') {
                let kv: Vec<&str> = parte.split('=').collect();
                println!("Parte: {:?}, kv: {:?}", parte, kv);
                if kv.len() == 2 {
                    match kv[0].trim() {
                        "fila" => { fila = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                        "columna" => { columna = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                        "valor" => { valor = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                        _ => {}
                    }
                }
            }
            println!("Resultado parseado: fila={}, columna={}, valor={}, parts_count={}", fila, columna, valor, parts_count);
            if parts_count == 3 {
                Some(matrix_partial::ResultadoCelda { fila, columna, valor })
            } else {
                None
            }
        })
        .collect();

    // Si no se obtuvieron celdas válidas, intentar decodificar y volver a intentar
    if resultados.is_empty() {
        let mut decoded_results_str = matrix_partial::url_decode(&results_str);
        decoded_results_str = decoded_results_str.replace("\n", "").replace("\r", "").replace(" ", "");
        println!("results_str normalizado (dec): {:?}", decoded_results_str);
        resultados = decoded_results_str
            .split(';')
            .filter_map(|celda_str| {
                if celda_str.is_empty() { return None; }
                let mut fila = 0;
                let mut columna = 0;
                let mut valor = 0;
                let mut parts_count = 0;
                for parte in celda_str.split(',') {
                    let kv: Vec<&str> = parte.split('=').collect();
                    println!("Parte (dec): {:?}, kv: {:?}", parte, kv);
                    if kv.len() == 2 {
                        match kv[0].trim() {
                            "fila" => { fila = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                            "columna" => { columna = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                            "valor" => { valor = kv[1].trim().parse().unwrap_or(0); parts_count += 1; },
                            _ => {}
                        }
                    }
                }
                println!("Resultado parseado (dec): fila={}, columna={}, valor={}, parts_count={}", fila, columna, valor, parts_count);
                if parts_count == 3 {
                    Some(matrix_partial::ResultadoCelda { fila, columna, valor })
                } else {
                    None
                }
            })
            .collect();
    }

    // Calcular dimensiones y construir la matriz final
    let (filas, columnas) = matrix_total::calcular_dimensiones_resultado(&matriz_a, &matriz_b);
    let matriz_resultado = matrix_total::construir_matriz_resultado(&resultados, filas, columnas);

    // Devolver el resultado como JSON
    let respuesta_json = matrix_total::matriz_a_json(&matriz_resultado);
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    HttpResponse::new(
        "HTTP/1.1".to_string(),
        200,
        headers,
        respuesta_json,
    )
}

fn invalid_request(contents: String) -> HttpResponse {
    let res = HttpResponse::new("HTTP/1.1".to_string(), 400, HashMap::new(), contents);
    res
}

fn valid_request(contents: String) -> HttpResponse {
    let res = HttpResponse::new("HTTP/1.1".to_string(), 200, HashMap::new(), contents);
    res
}
