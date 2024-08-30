# TAlgo: A Open version of Codeforces in Rust

TAlgo is a competitive programming platform inspired by Codeforces, implemented as a Rust workspace. This project aims to provide a robust, efficient, and user-friendly environment for coding competitions and practice.

## Project Structure

This workspace contains the following crates:

- `rustforces-core`: Core functionality and data structures
- `rustforces-web`: Web server and API endpoints
- `rustforces-judge`: Code execution and judging system
- `rustforces-cli`: Command-line interface for local testing

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

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rustforces.git
   cd rustforces
   ```

2. Set up the database and configure environment variables (see `.env.example`).

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the web server:
   ```
   cargo run -p rustforces-web
   ```

## Usage

- Access the web interface at `http://localhost:8080`
- Use the CLI for local testing:
  ```
  cargo run -p rustforces-cli -- submit problem_id solution.rs
  ```

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
