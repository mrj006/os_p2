mod server;
mod errors;

fn main() {
    server::create_server(7878);
}
