# Records API (Rust/Axum)

A lightweight REST API built with Rust and the Axum framework. It traces and stores arbitrary JSON payloads as local files for easy debugging and data collection.

## Features
- **Data Persistence**: Stores requests as JSON files in `./data`.
- **Traceability**: Supports custom `X-Trace-ID` headers.
- **Auto-Documentation**: Serves its own OpenAPI 3.0 specification in both YAML and JSON formats.

## Quickstart

### 1. Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)

### 2. Launch the Server
From the project root (`rustau` folder):
```bash
# Run the server
cargo run
```
*The server will start on port **8081**.*

## Usage Examples (CURL)

### Create a Record
```bash
curl -X POST http://localhost:8081/api/v1/records \
     -H "Content-Type: application/json" \
     -H "X-Trace-ID: my-custom-trace-id" \
     -d '{"app": "demo", "event": "user_login", "level": "info"}'
```
*Returns: `{"status":"success","uuid":"..."}`*

### Retrieve a Record
Replace `{uuid}` with the ID returned by the POST request:
```bash
curl -s http://localhost:8081/api/v1/records/{uuid}
```

### View API Specification
**YAML Format:**
```bash
curl -s http://localhost:8081/api/v1/openapi.yaml
```

**JSON Format:**
```bash
curl -s http://localhost:8081/api/v1/openapi.json
```

## Storage Structure
Stored records in `./data/*.json` follow this format:
```json
{
  "uuid": "unique-v4-id",
  "trace_id": "provided-or-generated-id",
  "timestamp": "UTC-iso-8601",
  "payload": { ... arbitrary json ... }
}
```
