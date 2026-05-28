<div align="center">

<!-- Animated SVG Logo / Banner -->
<svg width="900" height="200" viewBox="0 0 900 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#080d1a"/>
      <stop offset="100%" style="stop-color:#0d1526"/>
    </linearGradient>
    <linearGradient id="tealAmber" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" style="stop-color:#14b8a6"/>
      <stop offset="100%" style="stop-color:#f59e0b"/>
    </linearGradient>
    <filter id="glow">
      <feGaussianBlur stdDeviation="3" result="blur"/>
      <feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
  </defs>
  <rect width="900" height="200" fill="url(#bg)" rx="16"/>
  <!-- Grid lines -->
  <g opacity="0.06" stroke="#14b8a6" stroke-width="0.5">
    <line x1="0" y1="50" x2="900" y2="50"/><line x1="0" y1="100" x2="900" y2="100"/>
    <line x1="0" y1="150" x2="900" y2="150"/><line x1="150" y1="0" x2="150" y2="200"/>
    <line x1="300" y1="0" x2="300" y2="200"/><line x1="450" y1="0" x2="450" y2="200"/>
    <line x1="600" y1="0" x2="600" y2="200"/><line x1="750" y1="0" x2="750" y2="200"/>
  </g>
  <!-- Glow orb -->
  <circle cx="120" cy="100" r="60" fill="#0d9488" opacity="0.12">
    <animate attributeName="r" values="60;75;60" dur="4s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0.12;0.18;0.12" dur="4s" repeatCount="indefinite"/>
  </circle>
  <circle cx="780" cy="100" r="50" fill="#f59e0b" opacity="0.08">
    <animate attributeName="r" values="50;65;50" dur="5s" repeatCount="indefinite"/>
  </circle>
  <!-- Diamond icon -->
  <g transform="translate(68,100)" filter="url(#glow)">
    <polygon points="0,-32 22,0 0,32 -22,0" fill="none" stroke="url(#tealAmber)" stroke-width="2.5">
      <animateTransform attributeName="transform" type="rotate" values="0;360" dur="12s" repeatCount="indefinite"/>
    </polygon>
    <polygon points="0,-18 12,0 0,18 -12,0" fill="#14b8a6" opacity="0.5">
      <animate attributeName="opacity" values="0.5;0.9;0.5" dur="3s" repeatCount="indefinite"/>
    </polygon>
  </g>
  <!-- Title -->
  <text x="130" y="88" font-family="Georgia,serif" font-size="52" font-weight="900" fill="url(#tealAmber)" filter="url(#glow)">AKASHA</text>
  <!-- Subtitle -->
  <text x="132" y="118" font-family="'Courier New',monospace" font-size="14" fill="#94a3b8" letter-spacing="1">Temporal · Semantic · Causal Database</text>
  <!-- Tagline -->
  <text x="132" y="150" font-family="Georgia,serif" font-size="13" fill="#64748b" font-style="italic">
    "The eternal record — query across time, meaning, and causality"
  </text>
  <!-- Animated pulse dots -->
  <circle cx="820" cy="70" r="4" fill="#14b8a6">
    <animate attributeName="opacity" values="1;0.2;1" dur="2s" repeatCount="indefinite"/>
  </circle>
  <circle cx="840" cy="70" r="4" fill="#f59e0b">
    <animate attributeName="opacity" values="1;0.2;1" dur="2s" begin="0.5s" repeatCount="indefinite"/>
  </circle>
  <circle cx="860" cy="70" r="4" fill="#a78bfa">
    <animate attributeName="opacity" values="1;0.2;1" dur="2s" begin="1s" repeatCount="indefinite"/>
  </circle>
  <text x="810" y="94" font-family="monospace" font-size="9" fill="#64748b">45 tests</text>
  <text x="810" y="108" font-family="monospace" font-size="9" fill="#64748b">passing</text>
</svg>

---

[![CI](https://img.shields.io/github/actions/workflow/status/vignesh2027/akasha/ci.yml?branch=main&label=CI&style=flat-square&color=0d9488)](https://github.com/vignesh2027/akasha/actions)
[![Tests](https://img.shields.io/badge/tests-45%20passing-0d9488?style=flat-square)](https://github.com/vignesh2027/akasha)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-f59e0b?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-0d9488?style=flat-square)](LICENSE)
[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-14b8a6?style=flat-square)](https://vignesh2027.github.io/akasha)
[![crates.io](https://img.shields.io/badge/crates.io-coming%20soon-f59e0b?style=flat-square)](https://crates.io)

</div>

---

## What is Akasha?

**Akasha** is the world's first **Temporal-Semantic-Causal database** — an embedded, append-only, content-addressed data store that lets you query across **three orthogonal dimensions** simultaneously:

| Dimension | What it answers | How |
|-----------|----------------|-----|
| **⏱ Temporal** | *When* did it happen? | Nanosecond-precision BTree time index |
| **🔍 Semantic** | *What does it mean?* | Hand-rolled HNSW vector similarity index |
| **🔗 Causal** | *Why* did it happen? | Bidirectional causal DAG with BFS traversal |

> **Named after the Hindu concept of आकाश (akasha)** — the eternal, all-pervading medium that is said to record every event in the universe. In Akasha, every record is immutable, content-addressed, and queryable across all of time.

---

## The Problem No One Has Solved

Modern applications generate data with three intrinsic dimensions — but databases force you to pick one:

```
  You need:                  Current solution:
  ─────────────────────────────────────────────
  "Events from last week"  → Time-series DB (InfluxDB, TimescaleDB)
  "Semantically similar"   → Vector DB (Pinecone, Weaviate, Qdrant)
  "What caused this?"      → Graph DB (Neo4j, DGraph)
  ALL THREE AT ONCE        → ??? (doesn't exist)
```

**Akasha** answers all three in a single query, stored in a single engine, with a single query language.

---

## The Query That Doesn't Exist Elsewhere

```sql
FIND records
WHERE time BETWEEN "2024-01-01T00:00:00Z" AND "2024-12-31T23:59:59Z"
  AND similar_to embedding([0.91, 0.23, -0.14, ...]) WITH threshold 0.85
  AND caused_by "a3f8e2b1c4d9f6e2..." WITH depth 3
  AND tag environment = "production"
ORDER BY time DESC
LIMIT 25
```

> *"Give me all production events from 2024, semantically similar to this embedding, that were triggered (up to 3 hops) by this specific incident."*

This query spans **time**, **meaning**, and **causality** simultaneously. No existing database can do this.

---

## Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                    REST API  (Axum, HTTP/JSON)                  │
│          /v1/records · /v1/query · /v1/search · /v1/causal     │
├────────────────────────────────────────────────────────────────┤
│                  AQSL Query Engine                              │
│  Recursive Descent Parser → Logical Plan → Intersect + Rank    │
├───────────────┬──────────────────┬─────────────────────────────┤
│  ⏱ Temporal  │  🔍 Semantic      │  🔗 Causal                  │
│  Index        │  Index            │  Graph                      │
│               │                   │                             │
│ BTreeMap      │ HNSW Graph        │ Bidirectional               │
│ <i64,Vec<Id>> │ (hand-rolled)     │ Adjacency Lists             │
│ Range: O(logn)│ kNN: O(log n)     │ BFS: O(V+E)                 │
│ RwLock        │ DashMap (conc.)   │ DashMap (conc.)             │
├───────────────┴──────────────────┴─────────────────────────────┤
│              Storage Engine  (sled, pure Rust)                  │
│  Content-addressed · SHA-256 keys · Append-only · JSON values  │
│  Idempotent writes · Tamper-evident · Startup index rebuild     │
└────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

| Decision | Why |
|----------|-----|
| **Content-addressed storage (SHA-256)** | Idempotent writes — inserting the same record twice is a no-op. Records are tamper-evident by construction. |
| **Three independent indices** | Each dimension is queryable independently *and* in combination. The query engine intersects results at the record-ID level. |
| **HNSW from scratch** | No external vector library dependency. The implementation is tuned for Akasha's access pattern. M=16, ef=200 by default. |
| **Append-only log** | Never mutate historical data. Compliance-friendly. GDPR erasure supported via explicit delete. |
| **DashMap for concurrent indices** | Lock-free reads, fine-grained write locking. Akasha is designed for high concurrency. |
| **sled for storage** | Pure Rust. No external process. Embedded. ACID. Zero setup. |

---

## Installation

### Binary (CLI)

```bash
cargo install akasha
```

### Library

```toml
# Cargo.toml
[dependencies]
akasha = "0.1"
tokio = { version = "1", features = ["full"] }
```

### From Source

```bash
git clone https://github.com/vignesh2027/akasha
cd akasha
cargo build --release
./target/release/akasha --help
```

---

## Quick Start

### Start the server

```bash
akasha serve --db ./mydata --addr 0.0.0.0:7777
```

```
  ╔═══════════════════════════════════════╗
  ║        A K A S H A  v0.1.0           ║
  ║   Temporal · Semantic · Causal DB     ║
  ╚═══════════════════════════════════════╝

  Listening on http://0.0.0.0:7777
  Database : ./mydata
```

### Insert records (CLI)

```bash
# Insert a record
ID=$(akasha insert \
  --schema "payment.v2" \
  --data '{"amount": 99.99, "currency": "USD", "user_id": "u42"}')

echo "Stored: $ID"
# → Stored: a3f8e2b1c4d9f6e2...

# Insert a causally-related record
akasha insert \
  --schema "notification.v1" \
  --data '{"type": "payment_receipt", "user_id": "u42"}' \
  --causes "$ID"
```

### Insert via REST API

```bash
curl -X POST http://localhost:7777/v1/records \
  -H 'Content-Type: application/json' \
  -d '{
    "schema": "payment.v2",
    "data": {"amount": 99.99, "currency": "USD"},
    "embedding": [0.91, 0.23, -0.14, 0.05],
    "metadata": {"environment": "production", "region": "us-east-1"}
  }'

# → {"id": "a3f8e2b1c4d9f6e2..."}
```

### Query with AQSL

```bash
# CLI
akasha query "FIND records WHERE schema = \"payment.v2\" LIMIT 50"

# REST
curl -X POST http://localhost:7777/v1/query \
  -H 'Content-Type: application/json' \
  -d '{"aqsl": "FIND records WHERE time AFTER \"2024-01-01T00:00:00Z\" AND schema = \"payment.v2\" ORDER BY time DESC LIMIT 100"}'
```

---

## Rust Library Usage

```rust
use akasha::{Akasha, RecordBuilder};
use chrono::Utc;

#[tokio::main]
async fn main() -> akasha::Result<()> {
    // Open (or create) a database
    let db = Akasha::open("/data/akasha").await?;

    // Build and insert a record
    let record = RecordBuilder::new()
        .schema("event.v1")
        .data(serde_json::json!({
            "type": "user_signup",
            "user_id": "u42",
            "plan": "pro"
        }))
        .embedding(get_embedding("user_signup pro plan")) // your embedding fn
        .tag("environment", "production")
        .tag("region", "us-east-1")
        .build()?;

    let id = db.insert(record).await?;
    println!("Stored: {}", hex::encode(id));

    // Time-range query
    let recent = db.find_by_time(
        Utc::now() - chrono::Duration::hours(24),
        Utc::now(),
        100,
    ).await?;

    // Semantic similarity search
    let similar = db.find_similar(
        &get_embedding("user registration"),
        10,
        0.85, // cosine similarity threshold
    ).await?;

    // Causal chain traversal
    let effects = db.find_effects(&id, 5).await?;
    let causes  = db.find_causes(&id, 5).await?;

    // AQSL query
    let results = db.query(
        "FIND records \
         WHERE time AFTER \"2024-01-01T00:00:00Z\" \
           AND similar_to embedding([0.91, 0.23, ...]) WITH threshold 0.8 \
           AND caused_by \"a3f8...\" WITH depth 3 \
         LIMIT 25"
    ).await?;

    println!("Found {} records", results.len());
    Ok(())
}
```

---

## AQSL — Akasha Query Specification Language

AQSL is a composable, declarative query language with **first-class temporal, semantic, and causal clauses**. It is parsed by a zero-dependency, hand-written recursive descent parser.

### Grammar

```
query        := FIND records [WHERE clause (AND clause)*] [ORDER BY order] [LIMIT n] [OFFSET n]
clause       := time_clause | semantic_clause | causal_clause | schema_clause | tag_clause
time_clause  := time BETWEEN time_ref AND time_ref
              | time AFTER time_ref
              | time BEFORE time_ref
sem_clause   := similar_to embedding([f32, ...]) [WITH threshold f32]
causal_clause:= caused_by hex_id [WITH depth n]
              | causes hex_id [WITH depth n]
schema_clause:= schema = "name"
tag_clause   := tag key = "value"
time_ref     := "ISO-8601-string" | unix_nanos | NOW | NOW - nanos
order        := time ASC | time DESC | relevance
```

### Clause Reference

| Clause | Description | Example |
|--------|-------------|---------|
| `time BETWEEN a AND b` | Records in time range [a, b] | `time BETWEEN "2024-01-01" AND "2024-12-31"` |
| `time AFTER t` | Records after timestamp t | `time AFTER "2024-06-01T00:00:00Z"` |
| `time BEFORE t` | Records before timestamp t | `time BEFORE "2024-01-01T00:00:00Z"` |
| `similar_to embedding([...]) WITH threshold f` | Semantically similar records (cosine similarity ≥ threshold) | `similar_to embedding([0.1, 0.9]) WITH threshold 0.85` |
| `caused_by "id" WITH depth n` | Causal descendants of `id`, up to `n` hops | `caused_by "a3f8..." WITH depth 5` |
| `causes "id" WITH depth n` | Causal ancestors of `id`, up to `n` hops | `causes "a3f8..." WITH depth 3` |
| `schema = "name"` | Records with matching schema | `schema = "payment.v2"` |
| `tag key = "value"` | Records with matching metadata tag | `tag environment = "production"` |

### Query Examples

#### Incident RCA (Root Cause Analysis)

```sql
-- Find all events caused by a known bad deploy, in the last 6 hours
FIND records
WHERE caused_by "deploy-id-a3f8e2b1..." WITH depth 10
  AND time AFTER "2024-03-15T18:00:00Z"
  AND tag environment = "production"
ORDER BY time ASC
LIMIT 500
```

#### AI Memory Retrieval

```sql
-- Find memories semantically similar to current context, from last 30 days
FIND records
WHERE similar_to embedding([0.123, -0.456, 0.789, ...]) WITH threshold 0.88
  AND time AFTER "2024-02-01T00:00:00Z"
  AND schema = "memory.v1"
ORDER BY relevance
LIMIT 20
```

#### Full Three-Dimensional Query

```sql
FIND records
WHERE time BETWEEN "2024-01-01T00:00:00Z" AND "2024-12-31T23:59:59Z"
  AND similar_to embedding([0.91, 0.23, -0.14, 0.05, ...]) WITH threshold 0.82
  AND caused_by "root-incident-id..." WITH depth 5
  AND tag severity = "critical"
  AND schema = "alert.v2"
ORDER BY time DESC
LIMIT 50
```

---

## REST API Reference

Base URL: `http://localhost:7777`

### `GET /v1/stats`

Returns database statistics.

```json
{
  "total_records": 42847,
  "temporal_entries": 41203,
  "semantic_vectors": 38901,
  "causal_edges": 12445
}
```

### `POST /v1/records`

Insert a record.

**Request:**
```json
{
  "schema": "payment.v2",
  "data": { "amount": 99.99, "currency": "USD" },
  "timestamp": 1704067200000000000,
  "embedding": [0.91, 0.23, -0.14],
  "causes": ["a3f8e2b1c4d9f6e2..."],
  "metadata": { "environment": "production" }
}
```

**Response:**
```json
{ "id": "a3f8e2b1c4d9f6e2..." }
```

### `GET /v1/records/:id`

Retrieve a record by its 64-char hex content ID.

### `POST /v1/query`

Execute an AQSL query.

**Request:** `{ "aqsl": "FIND records WHERE schema = \"payment.v2\" LIMIT 10" }`

**Response:** `{ "count": 10, "records": [...] }`

### `GET /v1/search/time?from=&to=&limit=`

Search by time range (ISO 8601 strings).

### `POST /v1/search/similar`

Semantic similarity search.

**Request:** `{ "embedding": [0.1, 0.9, ...], "k": 10, "threshold": 0.8 }`

### `GET /v1/causal/:id/effects?depth=5`

Find all causal descendants of a record.

### `GET /v1/causal/:id/causes?depth=5`

Find all causal ancestors of a record.

### `GET /v1/causal/:from/path/:to`

Find the shortest causal path between two records.

**Response:** `{ "path": ["id1", "id2", "id3"], "length": 3 }`

---

## Record Schema

```rust
pub struct Record {
    /// Content-addressed ID: SHA-256(schema + data + timestamp)
    pub id: [u8; 32],

    /// Logical timestamp in nanoseconds since Unix epoch
    pub timestamp: i64,

    /// Wall-clock insertion time
    pub inserted_at: DateTime<Utc>,

    /// Schema name and version, e.g. "payment.v2"
    pub schema: String,

    /// Arbitrary JSON payload
    pub data: serde_json::Value,

    /// Optional semantic embedding vector
    pub embedding: Option<Vec<f32>>,

    /// Content IDs of causal predecessors
    pub causes: Vec<[u8; 32]>,

    /// Arbitrary key-value metadata
    pub metadata: HashMap<String, String>,
}
```

---

## Testing

```bash
# Run all 45 tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_test

# Run examples
cargo run --example basic_usage
cargo run --example causal_chain

# Benchmarks
cargo bench
```

### Test Coverage

| Module | Tests | Coverage |
|--------|-------|----------|
| `record` | 4 | Builder, ID determinism, validation |
| `storage` | 5 | Put/get, idempotency, scan, delete |
| `index::temporal` | 4 | Range, limit, before/after, remove |
| `index::semantic` | 4 | Cosine similarity, kNN, threshold |
| `graph::causal` | 5 | BFS, depth, path, roots/leaves |
| `query::parser` | 6 | All clause types, ORDER BY, errors |
| `integration` | 14 | Full end-to-end across all APIs |
| `doc tests` | 3 | API documentation examples |

---

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Insert (no embedding) | O(log n) | BTree insert + sled write |
| Insert (with embedding) | O(log n) | +HNSW insert (log n amortized) |
| Time range query | O(log n + k) | BTree range scan, k = results |
| Semantic kNN search | O(log n) | HNSW approximate, tunable recall |
| Causal descendants (BFS) | O(V + E) | V = visited nodes, E = edges |
| Shortest causal path | O(V + E) | BFS over causal DAG |
| Record retrieval by ID | O(1) | Direct hash lookup in sled |

---

## Use Cases

### 🔍 Incident Investigation & SRE

Akasha was designed for this. When a production incident occurs, you can:
1. Find all events semantically similar to the failure signature
2. Trace the full causal chain back to the root cause
3. Correlate with the exact time window

```sql
FIND records
WHERE caused_by "deploy-abc..." WITH depth 10
  AND similar_to embedding([error_embedding...]) WITH threshold 0.8
  AND time AFTER "2024-03-15T18:00:00Z"
ORDER BY time ASC LIMIT 500
```

### 🤖 AI Memory & RAG Systems

Store LLM conversation turns as records, with embeddings for semantic search and causal links from prompts to responses. Retrieve relevant memories with temporal and semantic filtering.

### 📊 Event Sourcing & Audit Logs

Immutable, content-addressed event log. Every event knows its cause. Replay any past system state by querying up to a specific timestamp.

### 🏥 Healthcare / Compliance

Track medical decisions with causal chains ("this treatment caused this outcome") and tamper-evident storage. GDPR compliance via explicit deletion API.

### 📈 Financial Systems

Model transaction causality (charge → refund → notification → report) with full temporal precision and semantic search over transaction descriptions.

---

## Contributing

Contributions are welcome! Please open an issue first to discuss major changes.

```bash
git clone https://github.com/vignesh2027/akasha
cd akasha
cargo test       # All tests must pass
cargo clippy     # No warnings
cargo fmt        # Code must be formatted
```

---

## Roadmap

- [ ] Persistent HNSW index (survive restarts without rebuild)
- [ ] Distributed mode with Raft consensus
- [ ] Python bindings (`pyakasha`)
- [ ] WebAssembly support for browser-side queries
- [ ] Query planner with cost-based optimization
- [ ] Compression for vector storage (Product Quantization)
- [ ] Streaming query results via SSE
- [ ] Schema registry and validation
- [ ] Snapshot/restore functionality

---

## Author

**Vignesh S** — CSE 2022–26, Takshashila University

- GitHub: [@vignesh2027](https://github.com/vignesh2027)
- Other projects: [SYNTHRON](https://github.com/vignesh2027/synthron) · [VORTEXRAG](https://github.com/vignesh2027/VORTEXRAG) · [rustkvd](https://github.com/vignesh2027/rustkvd) · [FluxDB](https://github.com/vignesh2027/-FLUXDB)

---

## License

MIT License — see [LICENSE](LICENSE) for details.

---

<div align="center">

**Akasha** — *because your data is three-dimensional, and your database should be too.*

[🌐 Website](https://vignesh2027.github.io/akasha) · [📦 GitHub](https://github.com/vignesh2027/akasha) · [🐛 Issues](https://github.com/vignesh2027/akasha/issues)

</div>
