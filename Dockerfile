# Builder 
FROM rust:latest AS builder

#instala targetMUSL dentro de la imagen
RUN rustup target add aarch64-apple-darwin 

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release


COPY src ./src
COPY archivos ./archivos

# hace que no depende de la versión de CLIBC del SO
RUN cargo install --path . --root /usr/local

# Runtime
FROM debian:latest

RUN apt-get update && apt-get install -y ca-certificates curl nano host iputils-ping nmap && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/os_p2 /usr/local/bin/os_p2
COPY --from=builder /usr/src/app/archivos /archivos

WORKDIR /

CMD ["/usr/local/bin/os_p2"] 