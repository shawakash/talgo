FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/sub_sweeper/Cargo.toml crates/sub_sweeper/
COPY crates/data_dust/Cargo.toml crates/data_dust/

RUN mkdir -p crates/sub_sweeper/src crates/data_dust/src \
    && echo "fn main() {}" > crates/sub_sweeper/src/main.rs \
    && echo "fn main() {}" > crates/data_dust/src/main.rs \

    RUN cargo fetch

RUN rm crates/sub_sweeper/src/main.rs crates/data_dust/src/main.rs

COPY crates/sub_sweeper/src ./crates/sub_sweeper/src
COPY crates/data_dust/src ./crates/data_dust/src

RUN cargo build --release -p sub_sweeper

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/sub_sweeper ./sub_sweeper

EXPOSE 8080

CMD ["./sub_sweeper"]
