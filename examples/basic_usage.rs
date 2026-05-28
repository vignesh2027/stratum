/// Demonstrates basic Akasha operations: insert, get, time-range query, and semantic search.
use akasha::{Akasha, RecordBuilder};
use chrono::Utc;

#[tokio::main]
async fn main() -> akasha::Result<()> {
    let db = Akasha::open_memory().await?;

    // --- Insert records with semantic embeddings ---
    let records = vec![
        ("payment.v1", serde_json::json!({"amount": 99.99, "currency": "USD", "status": "success"}), vec![1.0f32, 0.0, 0.0, 0.0]),
        ("payment.v1", serde_json::json!({"amount": 50.00, "currency": "EUR", "status": "failed"}),  vec![0.9f32, 0.1, 0.0, 0.0]),
        ("audit.v1",   serde_json::json!({"action": "login",  "user": "alice"}),                      vec![0.0f32, 1.0, 0.0, 0.0]),
        ("audit.v1",   serde_json::json!({"action": "logout", "user": "alice"}),                      vec![0.0f32, 0.9, 0.1, 0.0]),
        ("sensor.v1",  serde_json::json!({"temp": 72.3, "humidity": 45}),                             vec![0.0f32, 0.0, 1.0, 0.0]),
    ];

    let mut ids = Vec::new();
    let base_ts = Utc::now().timestamp_nanos_opt().unwrap_or(0);

    for (i, (schema, data, emb)) in records.into_iter().enumerate() {
        let record = RecordBuilder::new()
            .schema(schema)
            .data(data)
            .embedding(emb)
            .timestamp(base_ts + i as i64 * 1_000_000_000)
            .tag("source", "example")
            .build()?;
        let id = db.insert(record).await?;
        ids.push(id);
        println!("Inserted: {}", hex::encode(id));
    }

    // --- Time-range query ---
    println!("\n=== Time-range query (all records) ===");
    let from = chrono::DateTime::from_timestamp(0, 0).unwrap();
    let to = chrono::Utc::now() + chrono::Duration::hours(1);
    let results = db.find_by_time(from, to, 100).await?;
    println!("Found {} records in time range", results.len());

    // --- Semantic similarity search ---
    println!("\n=== Semantic search (similar to payment records) ===");
    let query_vec = vec![0.95f32, 0.05, 0.0, 0.0];
    let similar = db.find_similar(&query_vec, 5, 0.7).await?;
    for (rec, score) in &similar {
        println!("  score={:.3}  [{}]  {}", score, rec.schema, rec.data);
    }

    // --- AQSL query ---
    println!("\n=== AQSL query: payment records ===");
    let results = db.query("FIND records WHERE schema = \"payment.v1\" LIMIT 10").await?;
    for rec in &results {
        println!("  [{}] {}", rec.schema, rec.data);
    }

    // --- Database stats ---
    println!("\n=== Stats ===");
    let stats = db.stats().await;
    println!("  Total records : {}", stats.total_records);
    println!("  Temporal idx  : {}", stats.temporal_entries);
    println!("  Semantic vecs : {}", stats.semantic_vectors);

    Ok(())
}
