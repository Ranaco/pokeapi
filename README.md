# PokeAPI Rust Proxy 🐾

A blazingly fast Rust API server that acts as a proxy to the [PokeAPI](https://pokeapi.co/), built with [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/). Perfect for developers who want to integrate Pokémon data into their applications without directly hitting the external API.

## Features ✨

- 🚀 High-performance async Rust server
- 🛡️ Error handling with custom API errors
- 📊 Proxy endpoints for Pokémon, types, abilities, moves, and evolution chains
- 🔍 Query parameters for pagination (limit/offset)
- 🐛 Comprehensive logging with tracing
- 🧪 Unit tests included
- 📦 Easy to deploy and run

## Installation 🛠️

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (comes with Rust)

### Clone the Repository

```bash
git clone https://github.com/yourusername/pokeapi-rust-proxy.git
cd pokeapi-rust-proxy
```

### Install Dependencies

```bash
cargo build
```

This will download and compile all necessary dependencies.

## Running 🚀

### Development Mode

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000` by default.

### Custom Address

Set the `SERVER_ADDR` environment variable:

```bash
SERVER_ADDR=0.0.0.0:8080 cargo run
```

### With Logging

The app uses tracing for logging. Set `RUST_LOG` for custom log levels:

```bash
RUST_LOG=pokeapi=debug,tower_http=debug cargo run
```

## API Endpoints 📡

All endpoints proxy to PokeAPI v2. Responses are JSON objects matching the original API.

### Pokémon

- `GET /api/v2/pokemon` - List Pokémon (with optional `?limit=20&offset=0`)
- `GET /api/v2/pokemon/{identifier}` - Get specific Pokémon by ID or name

### Types

- `GET /api/v2/type` - List types (with optional `?limit=20&offset=0`)
- `GET /api/v2/type/{identifier}` - Get specific type by ID or name

### Abilities

- `GET /api/v2/ability` - List abilities (with optional `?limit=20&offset=0`)
- `GET /api/v2/ability/{identifier}` - Get specific ability by ID or name

### Moves

- `GET /api/v2/move` - List moves (with optional `?limit=20&offset=0`)
- `GET /api/v2/move/{identifier}` - Get specific move by ID or name

### Evolution Chains

- `GET /api/v2/evolution-chain` - List evolution chains (with optional `?limit=20&offset=0`)
- `GET /api/v2/evolution-chain/{id}` - Get specific evolution chain by ID

### Health Check

- `GET /health` - Simple health check endpoint returning "OK"

### Example Usage

```bash
# Get Pikachu
curl http://127.0.0.1:3000/api/v2/pokemon/pikachu

# List first 10 Pokémon
curl "http://127.0.0.1:3000/api/v2/pokemon?limit=10"

# Health check
curl http://127.0.0.1:3000/health
```

## Testing 🧪

Run the unit tests:

```bash
cargo test
```

Note: Integration tests are currently in `src/tests/` and compile but don't run via `cargo test`. They can be moved to a top-level `tests/` directory for full integration testing.

## Project Structure 📁

```
pokeapi/
├── src/
│   ├── main.rs          # Server entry point
│   ├── routes.rs        # Route definitions
│   ├── handlers/        # Request handlers
│   ├── client/          # PokeAPI HTTP client
│   ├── models/          # Data models (if any)
│   ├── errors.rs        # Error types
│   ├── utils.rs         # Utilities (tracing setup)
│   └── tests/           # Unit tests
├── Cargo.toml           # Dependencies and metadata
└── README.md            # This file
```

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- [PokeAPI](https://pokeapi.co/) for the amazing Pokémon data
- [Axum](https://github.com/tokio-rs/axum) for the excellent web framework
- The Rust community for making async programming fun and efficient

---

Made with ❤️ and lots of ☕ by the Rust community.</content>
<parameter name="filePath">/home/astar/Code/rust/pokeapi/README.md