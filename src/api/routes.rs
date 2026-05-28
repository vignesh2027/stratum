use crate::record::RecordBuilder;
use crate::{Akasha, DbStats};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub fn build_router(db: Arc<Akasha>) -> Router {
    Router::new()
        .route("/", get(health))
        .route("/v1/stats", get(stats))
        .route("/v1/records", post(insert_record))
        .route("/v1/records/:id", get(get_record))
        .route("/v1/query", post(query_records))
        .route("/v1/search/time", get(search_by_time))
        .route("/v1/search/similar", post(search_similar))
        .route("/v1/causal/:id/effects", get(causal_effects))
        .route("/v1/causal/:id/causes", get(causal_causes))
        .route("/v1/causal/:id/path/:to", get(causal_path))
        .layer(CorsLayer::permissive())
        .with_state(db)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "engine": "akasha",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn stats(State(db): State<Arc<Akasha>>) -> Json<DbStats> {
    Json(db.stats().await)
}

#[derive(Deserialize)]
struct InsertRequest {
    schema: String,
    data: serde_json::Value,
    timestamp: Option<i64>,
    embedding: Option<Vec<f32>>,
    causes: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
struct InsertResponse {
    id: String,
}

async fn insert_record(
    State(db): State<Arc<Akasha>>,
    Json(req): Json<InsertRequest>,
) -> Result<Json<InsertResponse>, (StatusCode, Json<serde_json::Value>)> {
    let mut builder = RecordBuilder::new()
        .schema(req.schema)
        .data(req.data);

    if let Some(ts) = req.timestamp {
        builder = builder.timestamp(ts);
    }
    if let Some(emb) = req.embedding {
        builder = builder.embedding(emb);
    }
    if let Some(meta) = req.metadata {
        for (k, v) in meta {
            builder = builder.tag(k, v);
        }
    }
    if let Some(causes) = req.causes {
        let mut cause_ids = Vec::new();
        for hex in causes {
            match hex::decode(&hex) {
                Ok(bytes) if bytes.len() == 32 => {
                    let mut id = [0u8; 32];
                    id.copy_from_slice(&bytes);
                    cause_ids.push(id);
                }
                _ => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({"error": format!("invalid cause ID: {hex}")})),
                    ));
                }
            }
        }
        builder = builder.caused_by(cause_ids);
    }

    match builder.build() {
        Ok(record) => match db.insert(record).await {
            Ok(id) => Ok(Json(InsertResponse { id: hex::encode(id) })),
            Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
        },
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

async fn get_record(
    State(db): State<Arc<Akasha>>,
    Path(id_hex): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let bytes = hex::decode(&id_hex)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid hex ID"}))))?;
    if bytes.len() != 32 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "ID must be 64 hex chars"}))));
    }
    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes);

    match db.get(&id).await {
        Ok(Some(rec)) => Ok(Json(serde_json::to_value(&rec).unwrap())),
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "record not found"})))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

#[derive(Deserialize)]
struct QueryRequest {
    aqsl: String,
}

async fn query_records(
    State(db): State<Arc<Akasha>>,
    Json(req): Json<QueryRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match db.query(&req.aqsl).await {
        Ok(records) => Ok(Json(serde_json::json!({
            "count": records.len(),
            "records": records,
        }))),
        Err(e) => Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

#[derive(Deserialize)]
struct TimeSearchParams {
    from: Option<String>,
    to: Option<String>,
    limit: Option<usize>,
}

async fn search_by_time(
    State(db): State<Arc<Akasha>>,
    Query(params): Query<TimeSearchParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    use chrono::{DateTime, Utc};
    let from: DateTime<Utc> = params.from
        .and_then(|s| s.parse().ok())
        .unwrap_or(DateTime::from_timestamp(0, 0).unwrap());
    let to: DateTime<Utc> = params.to
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(Utc::now);
    let limit = params.limit.unwrap_or(100);

    match db.find_by_time(from, to, limit).await {
        Ok(records) => Ok(Json(serde_json::json!({"count": records.len(), "records": records}))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

#[derive(Deserialize)]
struct SimilarRequest {
    embedding: Vec<f32>,
    k: Option<usize>,
    threshold: Option<f32>,
}

async fn search_similar(
    State(db): State<Arc<Akasha>>,
    Json(req): Json<SimilarRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let k = req.k.unwrap_or(10);
    let threshold = req.threshold.unwrap_or(0.7);
    match db.find_similar(&req.embedding, k, threshold).await {
        Ok(results) => {
            let items: Vec<_> = results.into_iter().map(|(r, score)| {
                serde_json::json!({"score": score, "record": r})
            }).collect();
            Ok(Json(serde_json::json!({"count": items.len(), "results": items})))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

#[derive(Deserialize)]
struct DepthParams {
    depth: Option<usize>,
}

async fn causal_effects(
    State(db): State<Arc<Akasha>>,
    Path(id_hex): Path<String>,
    Query(params): Query<DepthParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_hex(&id_hex)?;
    match db.find_effects(&id, params.depth.unwrap_or(5)).await {
        Ok(records) => Ok(Json(serde_json::json!({"count": records.len(), "records": records}))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

async fn causal_causes(
    State(db): State<Arc<Akasha>>,
    Path(id_hex): Path<String>,
    Query(params): Query<DepthParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let id = parse_id_hex(&id_hex)?;
    match db.find_causes(&id, params.depth.unwrap_or(5)).await {
        Ok(records) => Ok(Json(serde_json::json!({"count": records.len(), "records": records}))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()})))),
    }
}

async fn causal_path(
    State(db): State<Arc<Akasha>>,
    Path((from_hex, to_hex)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let from = parse_id_hex(&from_hex)?;
    let to = parse_id_hex(&to_hex)?;
    let path = db.causal.shortest_path(&from, &to);
    match path {
        Some(ids) => {
            let hex_ids: Vec<String> = ids.iter().map(hex::encode).collect();
            Ok(Json(serde_json::json!({"path": hex_ids, "length": hex_ids.len()})))
        }
        None => Err((StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "no causal path found"})))),
    }
}

fn parse_id_hex(hex_str: &str) -> Result<[u8; 32], (StatusCode, Json<serde_json::Value>)> {
    let bytes = hex::decode(hex_str)
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "invalid hex ID"}))))?;
    if bytes.len() != 32 {
        return Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "ID must be 64 hex chars"}))));
    }
    let mut id = [0u8; 32];
    id.copy_from_slice(&bytes);
    Ok(id)
}
