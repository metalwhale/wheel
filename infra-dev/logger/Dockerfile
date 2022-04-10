FROM rust:1.60

RUN apt-get update -y

# `lld` linker
RUN apt-get install -y lld

RUN rustup component add rustfmt clippy
RUN cargo install cargo-edit
