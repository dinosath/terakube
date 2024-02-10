FROM rust:latest


RUN rustup component add rustfmt
RUN cargo install loco-cli
RUN cargo install sea-orm-cli
RUN apt-get update && apt-get install -y buildah