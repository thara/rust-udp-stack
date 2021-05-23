FROM rust:1.52.1

# RUN apt-get update && apt-get install -y tcpdump
# RUN apt-get install -y strace

WORKDIR /app

COPY . .
RUN cargo build

ENTRYPOINT ["/app/target/debug/rust-udp-stack"]
