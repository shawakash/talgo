# TAlgo: An Open Version of Codeforces in Rust

![TAlgo User Design](statics/user_design.png)

TAlgo is a competitive programming platform inspired by Codeforces, implemented as a Rust workspace. This project aims to provide a robust, efficient, and user-friendly environment for coding competitions and practice.

## Project Structure

This workspace contains the following crates:

- `core`: Core backend functionality and data structures
- `data_dust`: Database functions and utilities
- `oxide_wave`: WebSocket implementation
- `sub_sweeper`: Worker for managing submission queue and judging

## Features

- User registration and authentication
- Problem submission and evaluation
- Real-time contest management
- Leaderboards and user ratings
- Problem set browsing and filtering
- Editorial and solution discussions

## Getting Started

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (for database)
- Redis (for caching and real-time features)
- Docker (for containerization)
- Kafka (for message queueing)
- Diesel CLI (for database migrations)

### Installation

1. Clone the repository:
   ```
    git clone https://github.com/shawakash/talgo.git
    cd talgo
   ```

2. Set up the database and configure environment variables (see `.env.example`).

3. Build the project:
   ```
    cargo build --release
   ```

4. Setup the enviroments:
    ```
      cp .env.example .env
    ```
    Edit the `.env` file and set the values accordingly.

5. Start some services (PostgreSQL, Redis, Kafka) using Docker:
   ```
    cd docker/services
    docker-compose up -d
   ```

6. Run the database migrations:
   ```
    cd crates/data_dust
    diesel migration run
    cd ../..
    ```

7. Start the server:
    ```
      cargo run --bin core
      cargo run --bin oxide_wave
    ```


## Usage

- Access the core backend api server at `http://localhost:8080`
- Access the web socket server at `http://localhost:8081`

## Development

To run tests across all crates:
```
cargo test --workspace
```

To format the code:
```
cargo fmt --all
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Codeforces](https://codeforces.com/)
- Built with [Rust](https://www.rust-lang.org/)
