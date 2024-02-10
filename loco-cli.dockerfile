# Use the official Rust image as a base
FROM rust:latest

RUN cargo install loco-cli
RUN cargo install sea-orm-cli
