use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use stratum::{Stratum, RecordBuilder};

fn bench_insert(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(Stratum::open_memory()).unwrap();

    let mut group = c.benchmark_group("insert");
    group.throughput(Throughput::Elements(1));

    group.bench_function("insert_no_embedding", |b| {
        let mut counter = 0i64;
        b.iter(|| {
            counter += 1;
            let record = RecordBuilder::new()
                .schema("bench.v1")
                .data(black_box(serde_json::json!({"i": counter})))
                .timestamp(counter)
                .build()
                .unwrap();
            rt.block_on(db.insert(record)).unwrap();
        });
    });

    group.bench_function("insert_with_embedding_128d", |b| {
        let embedding: Vec<f32> = (0..128).map(|i| i as f32 / 128.0).collect();
        let mut counter = 0i64;
        b.iter(|| {
            counter += 1;
            let record = RecordBuilder::new()
                .schema("bench.v1")
                .data(black_box(serde_json::json!({"i": counter})))
                .embedding(embedding.clone())
                .timestamp(counter)
                .build()
                .unwrap();
            rt.block_on(db.insert(record)).unwrap();
        });
    });

    group.finish();
}

fn bench_query(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let db = rt.block_on(async {
        let db = Stratum::open_memory().await.unwrap();
        let embedding_base: Vec<f32> = (0..128).map(|i| i as f32 / 128.0).collect();
        for i in 0..1000i64 {
            let mut emb = embedding_base.clone();
            emb[0] = i as f32 / 1000.0;
            let record = RecordBuilder::new()
                .schema("bench.v1")
                .data(serde_json::json!({"i": i}))
                .embedding(emb)
                .timestamp(i)
                .build()
                .unwrap();
            db.insert(record).await.unwrap();
        }
        db
    });

    let mut group = c.benchmark_group("query");

    for k in [1, 10, 50] {
        group.bench_with_input(BenchmarkId::new("semantic_search_k", k), &k, |b, &k| {
            let query: Vec<f32> = (0..128).map(|i| i as f32 / 128.0).collect();
            b.iter(|| {
                let results = rt.block_on(db.find_similar(black_box(&query), k, 0.0)).unwrap();
                black_box(results);
            });
        });
    }

    group.bench_function("time_range_1000_records", |b| {
        b.iter(|| {
            let results = rt.block_on(
                db.find_by_time(
                    chrono::DateTime::from_timestamp(0, 0).unwrap(),
                    chrono::Utc::now(),
                    100,
                )
            ).unwrap();
            black_box(results);
        });
    });

    group.bench_function("aqsl_schema_filter", |b| {
        b.iter(|| {
            let results = rt.block_on(
                db.query(black_box("FIND records WHERE schema = \"bench.v1\" LIMIT 100"))
            ).unwrap();
            black_box(results);
        });
    });

    group.finish();
}

criterion_group!(benches, bench_insert, bench_query);
criterion_main!(benches);
