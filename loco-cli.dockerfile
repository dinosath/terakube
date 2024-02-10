FROM rust:latest


RUN rustup component add rustfmt
RUN cargo install loco-cli
RUN cargo install sea-orm-cli
RUN apt -y install buildah