use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use crate::request::HttpRequest;
use super::request_queue::RequestQueue;

use crate::functions::{
    createfile::createfile,
    deletefile::deletefile,
    fibonacci::fibonacci,
    hash::hash,
    help::help,
    random::random,
    reverse::reverse,
    simulate::simulate,
    sleep::sleep,
    timestamp::timestamp,
    toupper::toupper,
};

const MAX_WORKERS: usize = 4;

#[derive(Clone)]
pub struct DynamicHandler {
    pub queue: RequestQueue,
    pub active_threads: Arc<AtomicUsize>,
}

impl DynamicHandler {
    pub fn new(queue: RequestQueue) -> Self {
        Self {
            queue,
            active_threads: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn start(&self) {
        let handler = self.clone();

        thread::spawn(move || loop {
            if handler.active_threads.load(Ordering::SeqCst) < MAX_WORKERS {
                let request = handler.queue.dequeue();
                let handler_clone = handler.clone();

                handler.active_threads.fetch_add(1, Ordering::SeqCst);

                thread::spawn(move || {
                    handle_request(request);
                    handler_clone.active_threads.fetch_sub(1, Ordering::SeqCst);
                });
            } else {
                // Espera
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
}

pub fn handle_request(req: HttpRequest) {
    if let Some(route) = req.uri.get(1) {
        match route.as_str() {
            "createfile" => {
                let name = req.params.get("name");
                let content = req.params.get("content");
                let repeat = req.params.get("repeat");

                if let (Some(name), Some(content), Some(repeat)) = (name, content, repeat) {
                    if let Ok(repeat_parsed) = repeat.parse::<u64>() {
                        match createfile(name, content, repeat_parsed) {
                            Ok(_) => println!("[createfile] Archivo '{}' creado exitosamente.", name),
                            Err(e) => eprintln!("[createfile] Error: {}", e),
                        }
                    }
                }
            }

            "deletefile" => {
                if let Some(name) = req.params.get("name") {
                    match deletefile(name) {
                        Ok(_) => println!("[deletefile] Archivo '{}' eliminado.", name),
                        Err(e) => eprintln!("[deletefile] Error: {}", e),
                    }
                }
            }

            "fibonacci" => {
                if let Some(num) = req.params.get("num") {
                    if let Ok(n) = num.parse::<u128>() {
                        let result = fibonacci(n);
                        println!("[fibonacci] fib({}) = {}", n, result);
                    }
                }
            }

            "hash" => {
                if let Some(text) = req.params.get("text") {
                    let result = hash(text);
                    println!("[hash] {}", result);
                }
            }

            "help" => {
                println!("[help]\n{}", help());
            }

            "random" => {
                let count = req.params.get("count");
                let min = req.params.get("min");
                let max = req.params.get("max");

                if let (Some(count), Some(min), Some(max)) = (count, min, max) {
                    if let (Ok(c), Ok(a), Ok(b)) = (
                        count.parse::<usize>(),
                        min.parse::<i32>(),
                        max.parse::<i32>(),
                    ) {
                        let values = random(c, a, b);
                        println!("[random] {:?}", values);
                    }
                }
            }

            "reverse" => {
                if let Some(text) = req.params.get("text") {
                    let result = reverse(text);
                    println!("[reverse] {}", result);
                }
            }

            "simulate" => {
                let seconds = req.params.get("seconds");
                let task = req.params.get("task");

                if let (Some(seconds), Some(task)) = (seconds, task) {
                    if let Ok(s) = seconds.parse::<u64>() {
                        let result = simulate(s, task);
                        println!("[simulate] {}", result);
                    }
                }
            }

            "sleep" => {
                if let Some(seconds) = req.params.get("seconds") {
                    if let Ok(s) = seconds.parse::<u64>() {
                        println!("[sleep] Esperando {} segundos...", s);
                        sleep(s);
                        println!("[sleep] Finalizado.");
                    }
                }
            }

            "timestamp" => {
                let result = timestamp();
                println!("[timestamp] {}", result);
            }

            "toupper" => {
                if let Some(text) = req.params.get("text") {
                    let result = toupper(text);
                    println!("[toupper] {}", result);
                }
            }

            _ => {
                println!("[404] Ruta '{}' no reconocida.", route);
            }
        }
    }
}

