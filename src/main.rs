mod server;
mod errors;
mod functions;
mod pool;
mod status;

fn main() {
    server::create_server(7878);
}
