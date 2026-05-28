use chrono::{Duration, Utc};
use stratum::{RecordBuilder, Stratum};

async fn fresh_db() -> Stratum {
    Stratum::open_memory().await.unwrap()
}

// ─── Insertion & Retrieval ──────────────────────────────────────────────────

#[tokio::test]
async fn insert_and_retrieve_by_id() {
    let db = fresh_db().await;
    let record = RecordBuilder::new()
        .schema("test.v1")
        .data(serde_json::json!({"x": 42}))
        .build()
        .unwrap();
    let id = db.insert(record).await.unwrap();
    let retrieved = db.get(&id).await.unwrap().unwrap();
    assert_eq!(retrieved.data["x"], 42);
}

#[tokio::test]
async fn insert_same_record_twice_is_idempotent() {
    let db = fresh_db().await;
    let record = RecordBuilder::new()
        .schema("test.v1")
        .data(serde_json::json!({"k": "v"}))
        .timestamp(12345)
        .build()
        .unwrap();
    let id1 = db.insert(record.clone()).await.unwrap();
    let id2 = db.insert(record).await.unwrap();
    assert_eq!(id1, id2);
    assert_eq!(db.stats().await.total_records, 1);
}

#[tokio::test]
async fn get_nonexistent_returns_none() {
    let db = fresh_db().await;
    let result = db.get(&[0u8; 32]).await.unwrap();
    assert!(result.is_none());
}

// ─── Temporal Index ─────────────────────────────────────────────────────────

#[tokio::test]
async fn find_by_time_range_returns_correct_records() {
    let db = fresh_db().await;
    let base = Utc::now();

    for i in 0..10i64 {
        let record = RecordBuilder::new()
            .schema("event.v1")
            .data(serde_json::json!({"i": i}))
            .at(base + Duration::hours(i))
            .build()
            .unwrap();
        db.insert(record).await.unwrap();
    }

    let from = base + Duration::hours(2);
    let to = base + Duration::hours(5);
    let results = db.find_by_time(from, to, 100).await.unwrap();
    // should include hours 2, 3, 4, 5 = 4 records
    assert_eq!(
        results.len(),
        4,
        "expected 4 records in [+2h, +5h], got {}",
        results.len()
    );
}

#[tokio::test]
async fn find_by_time_respects_limit() {
    let db = fresh_db().await;
    let base = Utc::now();
    for i in 0..20i64 {
        let r = RecordBuilder::new()
            .schema("event.v1")
            .data(serde_json::json!({"i": i}))
            .at(base + Duration::seconds(i))
            .build()
            .unwrap();
        db.insert(r).await.unwrap();
    }
    let results = db
        .find_by_time(base - Duration::seconds(1), base + Duration::hours(1), 5)
        .await
        .unwrap();
    assert_eq!(results.len(), 5);
}

// ─── Semantic Index ──────────────────────────────────────────────────────────

#[tokio::test]
async fn semantic_search_finds_nearest_neighbors() {
    let db = fresh_db().await;

    let r1 = RecordBuilder::new()
        .schema("doc.v1")
        .data(serde_json::json!({"topic": "finance"}))
        .embedding(vec![1.0, 0.0, 0.0, 0.0])
        .timestamp(1)
        .build()
        .unwrap();
    let r2 = RecordBuilder::new()
        .schema("doc.v1")
        .data(serde_json::json!({"topic": "health"}))
        .embedding(vec![0.0, 1.0, 0.0, 0.0])
        .timestamp(2)
        .build()
        .unwrap();
    let r3 = RecordBuilder::new()
        .schema("doc.v1")
        .data(serde_json::json!({"topic": "tech"}))
        .embedding(vec![0.0, 0.0, 1.0, 0.0])
        .timestamp(3)
        .build()
        .unwrap();

    db.insert(r1).await.unwrap();
    db.insert(r2).await.unwrap();
    db.insert(r3).await.unwrap();

    let results = db
        .find_similar(&[0.99, 0.01, 0.0, 0.0], 1, 0.8)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0.data["topic"], "finance");
}

#[tokio::test]
async fn semantic_search_with_threshold_filters_results() {
    let db = fresh_db().await;
    let r = RecordBuilder::new()
        .schema("doc.v1")
        .data(serde_json::json!({}))
        .embedding(vec![1.0, 0.0, 0.0, 0.0])
        .timestamp(1)
        .build()
        .unwrap();
    db.insert(r).await.unwrap();

    // Perfect match should pass
    let hits = db
        .find_similar(&[1.0, 0.0, 0.0, 0.0], 5, 0.99)
        .await
        .unwrap();
    assert_eq!(hits.len(), 1);

    // Orthogonal should fail
    let misses = db
        .find_similar(&[0.0, 1.0, 0.0, 0.0], 5, 0.5)
        .await
        .unwrap();
    assert_eq!(misses.len(), 0);
}

// ─── Causal Graph ───────────────────────────────────────────────────────────

#[tokio::test]
async fn causal_descendants_returns_full_chain() {
    let db = fresh_db().await;

    let a = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"A"}))
        .timestamp(1)
        .build()
        .unwrap();
    let a_id = db.insert(a).await.unwrap();

    let b = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"B"}))
        .timestamp(2)
        .caused_by(vec![a_id])
        .build()
        .unwrap();
    let b_id = db.insert(b).await.unwrap();

    let c = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"C"}))
        .timestamp(3)
        .caused_by(vec![b_id])
        .build()
        .unwrap();
    let c_id = db.insert(c).await.unwrap();

    let effects = db.find_effects(&a_id, 10).await.unwrap();
    let effect_ids: Vec<_> = effects.iter().map(|r| r.id).collect();
    assert!(effect_ids.contains(&b_id));
    assert!(effect_ids.contains(&c_id));
}

#[tokio::test]
async fn causal_ancestors_returns_predecessors() {
    let db = fresh_db().await;

    let root = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"root"}))
        .timestamp(1)
        .build()
        .unwrap();
    let root_id = db.insert(root).await.unwrap();

    let mid = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"mid"}))
        .timestamp(2)
        .caused_by(vec![root_id])
        .build()
        .unwrap();
    let mid_id = db.insert(mid).await.unwrap();

    let leaf = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":"leaf"}))
        .timestamp(3)
        .caused_by(vec![mid_id])
        .build()
        .unwrap();
    let leaf_id = db.insert(leaf).await.unwrap();

    let causes = db.find_causes(&leaf_id, 10).await.unwrap();
    let cause_ids: Vec<_> = causes.iter().map(|r| r.id).collect();
    assert!(cause_ids.contains(&mid_id));
    assert!(cause_ids.contains(&root_id));
}

#[tokio::test]
async fn causal_depth_is_respected() {
    let db = fresh_db().await;

    let a = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":1}))
        .timestamp(1)
        .build()
        .unwrap();
    let a_id = db.insert(a).await.unwrap();
    let b = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":2}))
        .timestamp(2)
        .caused_by(vec![a_id])
        .build()
        .unwrap();
    let b_id = db.insert(b).await.unwrap();
    let c = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":3}))
        .timestamp(3)
        .caused_by(vec![b_id])
        .build()
        .unwrap();
    let c_id = db.insert(c).await.unwrap();
    let d = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({"n":4}))
        .timestamp(4)
        .caused_by(vec![c_id])
        .build()
        .unwrap();
    let d_id = db.insert(d).await.unwrap();

    let effects_depth1 = db.find_effects(&a_id, 1).await.unwrap();
    assert_eq!(effects_depth1.len(), 1);

    let effects_depth2 = db.find_effects(&a_id, 2).await.unwrap();
    assert_eq!(effects_depth2.len(), 2);

    let all_effects = db.find_effects(&a_id, 10).await.unwrap();
    assert_eq!(all_effects.len(), 3);
    let _ = d_id;
}

// ─── SQSL Query Engine ───────────────────────────────────────────────────────

#[tokio::test]
async fn sqsl_schema_filter() {
    let db = fresh_db().await;
    for i in 0..5i64 {
        let r = RecordBuilder::new()
            .schema("payment.v1")
            .data(serde_json::json!({"i": i}))
            .timestamp(i)
            .build()
            .unwrap();
        db.insert(r).await.unwrap();
    }
    for i in 0..3i64 {
        let r = RecordBuilder::new()
            .schema("audit.v1")
            .data(serde_json::json!({"i": i}))
            .timestamp(100 + i)
            .build()
            .unwrap();
        db.insert(r).await.unwrap();
    }
    let results = db
        .query("FIND records WHERE schema = \"payment.v1\"")
        .await
        .unwrap();
    assert_eq!(results.len(), 5);
    assert!(results.iter().all(|r| r.schema == "payment.v1"));
}

#[tokio::test]
async fn sqsl_limit_is_applied() {
    let db = fresh_db().await;
    for i in 0..20i64 {
        let r = RecordBuilder::new()
            .schema("e.v1")
            .data(serde_json::json!({"i": i}))
            .timestamp(i)
            .build()
            .unwrap();
        db.insert(r).await.unwrap();
    }
    let results = db
        .query("FIND records WHERE schema = \"e.v1\" LIMIT 7")
        .await
        .unwrap();
    assert_eq!(results.len(), 7);
}

#[tokio::test]
async fn sqsl_tag_filter() {
    let db = fresh_db().await;
    let tagged = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({}))
        .tag("env", "production")
        .timestamp(1)
        .build()
        .unwrap();
    let untagged = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({}))
        .tag("env", "staging")
        .timestamp(2)
        .build()
        .unwrap();
    db.insert(tagged).await.unwrap();
    db.insert(untagged).await.unwrap();

    let results = db
        .query("FIND records WHERE tag env = \"production\"")
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].metadata.get("env").unwrap(), "production");
}

// ─── Stats ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn stats_reflect_insertions() {
    let db = fresh_db().await;
    assert_eq!(db.stats().await.total_records, 0);

    let r = RecordBuilder::new()
        .schema("e.v1")
        .data(serde_json::json!({}))
        .embedding(vec![1.0, 0.0, 0.0])
        .build()
        .unwrap();
    db.insert(r).await.unwrap();

    let stats = db.stats().await;
    assert_eq!(stats.total_records, 1);
    assert_eq!(stats.semantic_vectors, 1);
}
