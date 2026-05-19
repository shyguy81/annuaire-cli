# Copilot Instructions for annuaire-cli

## Project Overview
This is a Rust CLI application for managing a contacts directory (Annuaire de Contacts). It consumes a FastAPI backend API via OpenAPI spec and provides a command-line interface for contact operations.

## Architecture

### Core Components
- **main.rs** - CLI entry point using `clap` for argument parsing; defines command structure (list, create, health)
- **client.rs** - Generated HTTP client using `reqwest` for API communication; base URL configurable via `--api_url` global flag
- **models.rs** - Generated data structures from OpenAPI spec; auto-generated and should not be manually edited
- **scripts/** - Code generation tools:
  - `generate-client.py` - Generates client.rs and models.rs from OpenAPI spec
  - `download-openapi.sh` - Downloads OpenAPI spec from remote API

### Code Generation Pipeline
The client and models are **generated from OpenAPI**. Workflow:
1. Run `./scripts/download-openapi.sh [URL]` to fetch openapi.json from API (defaults to https://annuaire-api.demo.docker.dev)
2. Run `python3 scripts/generate-client.py` to regenerate Rust code from the spec
3. Rebuild and test

**Never manually edit** client.rs or models.rs — regenerate them when API schema changes.

## Build & Run

### Build
```bash
cargo build --release        # Full optimized build
cargo build                  # Debug build (faster)
```

### Run
```bash
cargo run -- --help                                    # Show all commands
cargo run -- list --skip 0 --limit 50                  # List contacts with pagination
cargo run -- create --name "John" --email "john@example.com" --phone "+33123456789"
cargo run -- health                                    # Check API health
```

Custom API URL:
```bash
cargo run -- --api_url "http://localhost:8000" list
```

## Language & Dependencies
- **Rust 1.92.0** - Main language (see Cargo.toml for dependencies)
- **Key crates:**
  - `clap` - CLI argument parsing with derive macros
  - `reqwest` - Async HTTP client
  - `tokio` - Async runtime
  - `serde`/`serde_json` - Serialization
  - `anyhow` - Error handling

## Key Conventions

### Error Handling
- Use `anyhow::Result<T>` for fallible functions in main.rs
- Generated client methods return `Result<String, reqwest::Error>`
- The `?` operator bubbles errors up to main's Result type

### Async Code
- All HTTP client methods are async (`pub async fn`)
- Use `#[tokio::main]` on main() for async runtime
- Await HTTP calls with `.await`

### CLI Structure
- Use `derive(Parser)` and `derive(Subcommand)` macros from clap
- Global flags like `--api_url` go on the Cli struct
- Command-specific arguments go on variant fields in Commands enum
- Commands are matched in main with `match cli.command`

### Generated Code Comments
- `client.rs` has comment: `// Generated API Client - Run: python3 scripts/generate-client.py`
- `models.rs` has comment: `// Generated from OpenAPI - Run: python3 scripts/generate-client.py`
- These are reminders to regenerate if API schema changes

## Common Tasks

### Adding a New CLI Command
1. Add variant to `Commands` enum in main.rs
2. Add handler in the match statement in main()
3. Regenerate models/client if new API endpoints or data structures are needed

### Updating for API Changes
1. Download fresh OpenAPI spec: `./scripts/download-openapi.sh`
2. Regenerate: `python3 scripts/generate-client.py`
3. Update main.rs to handle new types or methods

### Testing API Connectivity
```bash
cargo run -- health          # Returns API health status
```

## Directory Structure
```
.
├── src/
│   ├── main.rs            # CLI commands and entry point
│   ├── client.rs          # Generated HTTP client (don't edit)
│   └── models.rs          # Generated data structures (don't edit)
├── scripts/
│   ├── generate-client.py # OpenAPI to Rust code generator
│   └── download-openapi.sh# Fetch API spec
├── openapi.json           # API specification (downloaded)
├── Cargo.toml             # Dependencies and project config
└── Cargo.lock             # Locked dependency versions
```

## Notes
- The CLI defaults to `https://annuaire-api.demo.docker.dev` for API URL
- Client methods currently return raw `String` — type conversions to structs can be added as needed
- Commands are in French (nom, email, telephone) reflecting the project's French context
