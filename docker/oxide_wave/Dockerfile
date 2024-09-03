FROM rust:latest AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/oxide_wave/Cargo.toml crates/oxide_wave/
COPY crates/data_dust/Cargo.toml crates/data_dust/

RUN mkdir -p crates/oxide_wave/src crates/data_dust/src \
    && echo "fn main() {}" > crates/oxide_wave/src/main.rs \
    && echo "fn main() {}" > crates/data_dust/src/main.rs \

    RUN cargo fetch

RUN rm crates/oxide_wave/src/main.rs crates/data_dust/src/main.rs

COPY crates/oxide_wave/src ./crates/oxide_wave/src
COPY crates/data_dust/src ./crates/data_dust/src

RUN cargo build --release -p oxide_wave

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=build /app/target/release/oxide_wave ./oxide_wave

EXPOSE 8080

CMD ["./oxide_wave"]
