mod distributed;
mod errors;
mod functions;
mod models;
mod pool;
mod server;
mod status;
mod redis_comm;

use std::env;

fn main() {
    // Lee la variable de entorno para determinar el rol
    let role = env::var("APP_ROLE").unwrap_or_else(|_| "standalone".to_string());

    println!("Iniciando aplicación en modo: {}", role);

    match role.as_str() {
        "master" => {
            // Lógica del Master
            
            if let Ok(redis_url) = env::var("REDIS_URL") {
                 println!("Conectando a Redis en: {}", redis_url);
            }

        }
        "slave" => {
            // Lógica del Slave
            let master_url = env::var("MASTER_URL").expect("La variable MASTER_URL es necesaria para el slave");
            
            // Aquí deberíá ir el request al endpoint del master.

        }
        _ => {
            println!("Rol no especificado. Corriendo para desarrollo local.");
        }
    }

    let port_str = env::var("PORT").unwrap_or_else(|_| "7878".to_string());
    let port: u16 = port_str.parse().expect("La variable de entorno PORT debe ser un número válido");
    
    println!("Iniciando servidor en el puerto {}", port);
    server::server::create_server(port);
}
