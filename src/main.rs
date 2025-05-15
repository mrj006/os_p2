mod server;
mod errors;
mod functions;
mod pool;

fn main() {
    server::create_server(7878);
}
