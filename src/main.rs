mod distributed;
mod errors;
mod functions;
mod models;
mod pool;
mod server;
mod status;

fn main() {
    server::server::create_server(7878);
}
