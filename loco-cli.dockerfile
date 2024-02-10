FROM rust:latest

RUN cargo install loco-cli
RUN cargo install sea-orm-cli
