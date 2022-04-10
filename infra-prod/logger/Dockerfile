FROM rust:1.60-slim

RUN apt-get update -y

# `lld` linker
RUN apt-get install -y lld

COPY . /usr/src/logger/
RUN cd /usr/src/logger/ && \
    cargo build --release && \
    cp ./target/release/logger /usr/local/bin/wheel-logger && \
    cd .. && \
    rm -rf ./logger/
