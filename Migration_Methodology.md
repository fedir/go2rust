# Methodology: Rapid & Safe Go to Rust Migration

This guide outlines the specific methodology used to migrate the Records API from Go to Rust, ensuring 100% API compatibility and data parity.

## 1. Analysis & Mapping Phase
Before writing any code, map the Go components to their Rust equivalents to maintain architectural consistency.

| Go Component | Rust Strategy |
| :--- | :--- |
| **Gin Gonic** | **Axum** (State-of-the-art, uses Tokio, similar routing logic). |
| **Struct Tags** (`json:"uuid"`) | **Serde** (`#[derive(Serialize, Deserialize)]`). |
| **Interface{}** | `serde_json::Value` (for arbitrary JSON payloads). |
| **Defer/Error handling** | `Result<T, E>` and `match` statements. |
| **Standard Library `os`** | `std::fs` and `tokio::fs` (if async I/O is prioritized). |

## 2. Structural Replication (The "Mirror" Approach)
To ensure a successful rewrite, follow the same project layout and data flow:

1.  **Define Shared Models First**: Recreate the Go `structs` in Rust using `serde`. Use `#[serde(rename_all = "snake_case")]` if the Go tags follow that convention to avoid manual renaming.
2.  **Match Route Patterns**:
    *   Go: `v1.GET("/records/:uuid", ...)`
    *   Rust: `.route("/api/v1/records/{uuid}", get(...))`
3.  **Preserve Side Effects**: If the Go app writes to `./data`, the Rust app must use the exact same directory, naming convention, and indentation (e.g., `serde_json::to_string_pretty`).

## 3. Implementation Workflow

### Step A: Dependency Scaffolding
Start with a minimal `Cargo.toml`. For a REST API, the "Gold Standard" stack is:
- `axum` + `tokio` (Web framework & Runtime)
- `serde` + `serde_json` (Serialization)
- `chrono` (DateTime handling)
- `uuid` (ID generation)

### Step B: The Handler Translation
Translate Go handlers line-by-line, focusing on behavior rather than syntax:
- **Header Extraction**: Replace Gin's `c.GetHeader` with Axum's `HeaderMap` extractor.
- **JSON Binding**: Replace `c.ShouldBindJSON` with Axum's `Json<T>` extractor.
- **Status Codes**: Map `http.StatusCreated` to `StatusCode::CREATED`.

### Step C: Parity Validation
Perform a "Header-to-Body" check:
1.  **Body Parity**: Use `diff` on the JSON output of both apps for the same input.
2.  **Header Parity**: Compare `Content-Type` and custom headers (like `X-Trace-ID`).
3.  **Storage Parity**: Verify that files created by Rust can be read by Go, and vice versa.

## 4. Tips for Speed
- **Use `serde_json::Value` for migration**: If the Go app handles "arbitrary JSON," don't try to define strict types in Rust immediately. Use `Value` to get the API running first, then refine types later.
- **Automate YAML/JSON conversion**: If the Go app uses a library like `go-yaml` for dynamic conversion, use `serde_yaml` and `serde_json` together in Rust to achieve the same result in ~5 lines of code.
- **Leverage `IntoResponse`**: Rust's Axum allows you to return tuples like `(StatusCode, Json(Value))`, which closely mirrors Gin's `c.JSON(code, obj)`.

## 5. Common Pitfalls to Avoid
- **Linker Issues**: In restricted environments, always verify your C compiler (`cc`/`gcc`) and set `RUSTFLAGS="-C linker=/usr/bin/gcc"` if necessary.
- **Trailing Newlines**: Go's `json.Marshal` and Rust's `serde_json` may differ in trailing whitespace. If exact byte-parity is required, use a custom buffer.
- **Path Syntax**: Be aware of framework differences (e.g., Gin's `:uuid` vs. Axum's `{uuid}`).
