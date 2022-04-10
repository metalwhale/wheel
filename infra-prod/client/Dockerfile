FROM rust:1.60-slim

RUN apt-get update -y

# `lld` linker
RUN apt-get install -y lld

# `openssl-sys` crate
RUN apt-get install -y pkg-config libssl-dev

COPY . /usr/src/client/
RUN cd /usr/src/client/ && \
    cargo build --release && \
    cp ./target/release/client /usr/local/bin/wheel-client && \
    cd .. && \
    rm -rf ./client/
