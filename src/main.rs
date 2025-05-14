mod server;
mod errors;
mod functions;

fn main() {
    server::create_server(7878);
}
