use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

const DATA_DIR: &str = "./data";

#[derive(Serialize, Deserialize, Debug)]
struct StoredRecord {
    uuid: String,
    trace_id: String,
    timestamp: DateTime<Utc>,
    payload: Value,
}

#[tokio::main]
async fn main() {
    // 1. Initialize data directory
    if let Err(e) = fs::create_dir_all(DATA_DIR) {
        eprintln!("failed to initialize data directory: {}", e);
        std::process::exit(1);
    }

    // 2. Setup router
    let app = Router::new()
        .route("/api/v1/records", post(create_record))
        .route("/api/v1/records/{uuid}", get(get_record_by_uuid))
        .route("/api/v1/openapi.yaml", get(get_openapi_yaml))
        .route("/api/v1/openapi.json", get(get_openapi_json));

    // 3. Start server
    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server starting on {}...", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn create_record(
    header_map: header::HeaderMap,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let trace_id = header_map
        .get("X-Trace-ID")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let new_uuid = Uuid::new_v4().to_string();
    let record = StoredRecord {
        uuid: new_uuid.clone(),
        trace_id,
        timestamp: Utc::now(),
        payload,
    };

    let file_path = PathBuf::from(DATA_DIR).join(format!("{}.json", new_uuid));
    let file_data = match serde_json::to_string_pretty(&record) {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to encode record"})),
            )
                .into_response()
        }
    };

    if let Err(_) = fs::write(file_path, file_data) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "failed to save record"})),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "uuid": new_uuid,
        })),
    )
        .into_response()
}

async fn get_record_by_uuid(Path(id): Path<String>) -> impl IntoResponse {
    if Uuid::parse_str(&id).is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "invalid uuid format"})),
        )
            .into_response();
    }

    let file_path = PathBuf::from(DATA_DIR).join(format!("{}.json", id));

    if !file_path.exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "record not found"})),
        )
            .into_response();
    }

    let file_data = match fs::read_to_string(file_path) {
        Ok(data) => data,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to read record"})),
            )
                .into_response()
        }
    };

    let record: StoredRecord = match serde_json::from_str(&file_data) {
        Ok(rec) => rec,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "corrupted record data"})),
            )
                .into_response()
        }
    };

    Json(record).into_response()
}

async fn get_openapi_yaml() -> impl IntoResponse {
    match fs::read_to_string("openapi.yaml") {
        Ok(content) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/yaml")],
            content,
        )
            .into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "failed to read openapi specification"})),
        )
            .into_response(),
    }
}

async fn get_openapi_json() -> impl IntoResponse {
    let yaml_content = match fs::read_to_string("openapi.yaml") {
        Ok(content) => content,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to read openapi specification"})),
            )
                .into_response()
        }
    };

    let data: Value = match serde_yaml::from_str(&yaml_content) {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "failed to parse yaml"})),
            )
                .into_response()
        }
    };

    Json(data).into_response()
}
