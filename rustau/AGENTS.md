# Records API - Rust Implementation

REST API in Rust (Axum) for receiving, tracing, and storing JSON data. This is a port of the original Go implementation.

## Architecture
- **Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Storage**: Local JSON files in `./data/`
- **Specification**: OpenAPI 3.0 (`openapi.yaml`)

## Endpoints

### 1. Data Management
- `POST /api/v1/records`: Create a record.
    - Optional Header: `X-Trace-ID`
    - Payload: Arbitrary JSON
- `GET /api/v1/records/{uuid}`: Retrieve a stored record.

### 2. Auto-documentation
- `GET /api/v1/openapi.yaml`: Specification in YAML format.
- `GET /api/v1/openapi.json`: Specification in JSON format (dynamic conversion).

## Development
Run the server:
```bash
cargo run
```

Stored files in the `./data` directory follow this structure:
- `uuid`: Unique identifier (v4).
- `trace_id`: Traceability ID (from header or auto-generated).
- `timestamp`: Reception time (ISO8601 UTC).
- `payload`: Raw received data.
