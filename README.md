# Records API: Go to Rust Migration & Comparison

This repository contains two implementations of the **Records API**: the original version written in **Go** and a modern, high-performance migration written in **Rust**. The project serves as a practical demonstration of migrating a RESTful microservice between languages while maintaining 100% API compatibility and data parity.

## Project Overview

The **Records API** is a lightweight service designed to receive, trace, and persist arbitrary JSON payloads.

### Core Features
- **Data Persistence**: Incoming JSON data is saved to local files named with unique UUIDs.
- **Traceability**: Support for `X-Trace-ID` headers to link related requests.
- **Self-Documentation**: Built-in endpoints to serve its own OpenAPI 3.0 specification in both YAML and JSON formats.
- **Data Integrity**: Uses standard ISO8601 UTC timestamps and validated UUIDs.

---

## Repository Structure

- `goau/`: Original Go implementation using the [Gin](https://github.com/gin-gonic/gin) framework.
- `rustau/`: Migrated Rust implementation using [Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).
- `_testing/`: Collection of test results, curl logs, and parity checks between the two versions.
- `Comparison_Report.md`: Detailed analysis of the migration results, performance findings, and functional parity.
- `Migration_Methodology.md`: Documentation of the strategies used for a safe and rapid rewrite.

---

## Technology Stack Comparison

| Component | Go Implementation (`goau`) | Rust Implementation (`rustau`) |
| :--- | :--- | :--- |
| **Language** | Go 1.20+ | Rust (Stable) |
| **Web Framework** | Gin Gonic | Axum |
| **Runtime** | Go Runtime | Tokio (Async) |
| **Serialization** | `encoding/json` | `serde` / `serde_json` |
| **UUIDs** | `google/uuid` | `uuid` crate |

---

## API Endpoints

Both implementations expose the following identical endpoints:

| Endpoint | Method | Description |
| :--- | :--- | :--- |
| `/api/v1/records` | `POST` | Create a new record from any JSON payload. |
| `/api/v1/records/{uuid}` | `GET` | Retrieve a specific stored record. |
| `/api/v1/openapi.yaml` | `GET` | Serve the OpenAPI 3.0 spec in YAML. |
| `/api/v1/openapi.json` | `GET` | Serve the OpenAPI 3.0 spec in JSON (dynamic conversion). |

---

## Getting Started

### Prerequisites
- [Go](https://golang.org/doc/install) (for `goau`)
- [Rust & Cargo](https://rustup.rs/) (for `rustau`)

### Running the Go Version (`goau`)
```bash
cd goau
go run main.go
```
*Note: The server starts on port `8081`.*

### Running the Rust Version (`rustau`)
```bash
cd rustau
cargo run
```
*Note: The server starts on port `8081`.*

---

## Migration Results & Findings

As documented in the `Comparison_Report.md`, the migration achieved:
1.  **100% Functional Parity**: All endpoints return identical structures and status codes.
2.  **Data Compatibility**: The Rust version can read data files created by the Go version and vice-versa.
3.  **Improved Error Handling**: The Rust implementation provides more granular feedback for malformed JSON inputs.
4.  **Type Safety**: Enhanced memory safety and compile-time checks through Rust's robust type system.

For a deep dive into the migration process, see [Migration Methodology](./Migration_Methodology.md).
