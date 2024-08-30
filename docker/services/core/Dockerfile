FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml crates/core/
COPY crates/data_dust/Cargo.toml crates/data_dust/

RUN mkdir -p crates/core/src crates/data_dust/src \
    && echo "fn main() {}" > crates/core/src/main.rs \
    && echo "fn main() {}" > crates/data_dust/src/main.rs \

    RUN cargo fetch

RUN rm crates/core/src/main.rs crates/data_dust/src/main.rs

COPY crates/core/src ./crates/core/src
COPY crates/data_dust/src ./crates/data_dust/src

RUN cargo build --release -p core

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/core ./core

EXPOSE 8080

CMD ["./core"]
