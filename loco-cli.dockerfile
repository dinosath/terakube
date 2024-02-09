# Use the official Rust image as a base
FROM rust:latest

# Install the required CLI tools using cargo
RUN cargo install loco-cli \
    && cargo install sea-orm-cli
