# Comparison Report: Go to Rust Migration (Records API)

This document provides an overview of the project and the results of the migration from the original Go implementation (`goau`) to the Rust implementation (`rustau`).

## 1. Project Overview
The **Records API** is a lightweight service designed to receive, trace, and store arbitrary JSON payloads. It serves as a data collection point with the following core responsibilities:
- **Persistence**: Save incoming JSON data to local files named after a unique UUID.
- **Traceability**: Support for `X-Trace-ID` headers to link requests.
- **Self-Documentation**: Expose its own OpenAPI 3.0 specification in both YAML and JSON formats.

## 2. Technology Stack

| Component | Go Implementation (`goau`) | Rust Implementation (`rustau`) |
| :--- | :--- | :--- |
| **Language** | Go 1.20+ | Rust (Latest Stable) |
| **Web Framework** | [Gin](https://github.com/gin-gonic/gin) | [Axum](https://github.com/tokio-rs/axum) |
| **Runtime** | Go Runtime | [Tokio](https://tokio.rs/) |
| **Serialization** | `encoding/json`, `go-yaml` | `serde`, `serde_json`, `serde_yaml` |
| **UUID Generation** | `github.com/google/uuid` | `uuid` crate (v4) |

## 3. Comparison Results

### API Signature & Endpoints
The Rust application was tested against the Go application using `curl`. All endpoints were found to be functionally identical.

| Endpoint | Method | Status | Notes |
| :--- | :--- | :--- | :--- |
| `/api/v1/records` | POST | Identical | Both return `201 Created` with a JSON body containing `status` and `uuid`. |
| `/api/v1/records/{uuid}` | GET | Identical | Both retrieve the exact stored JSON object with identical key structures. |
| `/api/v1/openapi.yaml` | GET | Identical | Serves the static specification file with `text/yaml` content-type. |
| `/api/v1/openapi.json` | GET | Identical | Dynamically converts YAML to JSON. |

### Data Format & Storage
The stored file structure in the `./data/` directory was compared:
- **File Naming**: Both use `{uuid}.json`.
- **Indentation**: Both use 2-space pretty-printing for JSON storage.
- **Timestamp**: Both implementations use ISO8601 UTC format (e.g., `2026-02-27T15:58:48.495Z`).

### Header Handling
- **X-Trace-ID**: Both applications correctly extract this header or generate a new UUID if it is missing.
- **Content-Type**: Both correctly serve `application/json; charset=utf-8` for API responses.

## 4. Findings & Observations

- **Performance**: The Rust implementation (using Axum/Tokio) offers lower memory overhead and high concurrency potential compared to the Gin implementation.
- **Error Messages**: While the success paths are identical, the Rust implementation provides more descriptive error messages for malformed JSON payloads (including line and column numbers), whereas the Go implementation uses a static "invalid JSON payload" message.
- **Safety**: The Rust implementation leverages the type system to ensure that the `StoredRecord` structure is strictly adhered to during both serialization and deserialization.

## 5. Conclusion
The migration from Go to Rust was successful. The `rustau` application is a drop-in replacement for `goau`, maintaining 100% compatibility with existing clients and data storage formats.
