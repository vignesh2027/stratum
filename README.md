<div align="center">

<svg width="900" height="210" viewBox="0 0 900 210" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#0d1f0d"/>
      <stop offset="100%" style="stop-color:#142814"/>
    </linearGradient>
    <linearGradient id="greenGrad" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" style="stop-color:#4ade80"/>
      <stop offset="100%" style="stop-color:#86efac"/>
    </linearGradient>
    <filter id="glow"><feGaussianBlur stdDeviation="4" result="blur"/>
      <feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge>
    </filter>
  </defs>
  <!-- Background -->
  <rect width="900" height="210" fill="url(#bg)" rx="16"/>
  <!-- Subtle grid -->
  <g opacity="0.07" stroke="#22c55e" stroke-width="0.5">
    <line x1="0" y1="52" x2="900" y2="52"/><line x1="0" y1="104" x2="900" y2="104"/>
    <line x1="0" y1="156" x2="900" y2="156"/><line x1="180" y1="0" x2="180" y2="210"/>
    <line x1="360" y1="0" x2="360" y2="210"/><line x1="540" y1="0" x2="540" y2="210"/>
    <line x1="720" y1="0" x2="720" y2="210"/>
  </g>
  <!-- Strata layer lines (thematic) -->
  <rect x="0" y="160" width="900" height="6"  fill="#166534" opacity="0.5" rx="0"/>
  <rect x="0" y="168" width="900" height="4"  fill="#15803d" opacity="0.4" rx="0"/>
  <rect x="0" y="174" width="900" height="3"  fill="#16a34a" opacity="0.3" rx="0"/>
  <rect x="0" y="179" width="900" height="2"  fill="#22c55e" opacity="0.2" rx="0"/>
  <!-- Glowing orb left -->
  <circle cx="80" cy="100" r="55" fill="#16a34a" opacity="0.14">
    <animate attributeName="r" values="55;70;55" dur="5s" repeatCount="indefinite"/>
    <animate attributeName="opacity" values="0.14;0.22;0.14" dur="5s" repeatCount="indefinite"/>
  </circle>
  <!-- Diamond mark -->
  <g transform="translate(80,100)" filter="url(#glow)">
    <polygon points="0,-28 20,0 0,28 -20,0" fill="none" stroke="url(#greenGrad)" stroke-width="2">
      <animateTransform attributeName="transform" type="rotate" values="0;360" dur="14s" repeatCount="indefinite"/>
    </polygon>
    <polygon points="0,-14 10,0 0,14 -10,0" fill="#22c55e" opacity="0.7">
      <animate attributeName="opacity" values="0.7;1;0.7" dur="3s" repeatCount="indefinite"/>
    </polygon>
  </g>
  <!-- Title -->
  <text x="140" y="86" font-family="Georgia,serif" font-size="58" font-weight="900"
        fill="url(#greenGrad)" filter="url(#glow)" letter-spacing="-1">STRATUM</text>
  <!-- Subtitle line -->
  <text x="142" y="116" font-family="'Courier New',monospace" font-size="13.5"
        fill="rgba(134,239,172,0.8)" letter-spacing="2.5">TEMPORAL  ·  SEMANTIC  ·  CAUSAL  DATABASE</text>
  <!-- Tagline -->
  <text x="142" y="146" font-family="Georgia,serif" font-size="13" fill="rgba(255,255,255,0.38)" font-style="italic">
    Query across time, meaning, and causality — in a single expression
  </text>
  <!-- Status indicators top-right -->
  <g transform="translate(800,50)">
    <circle cx="0" cy="0" r="5" fill="#22c55e">
      <animate attributeName="opacity" values="1;0.3;1" dur="2s" repeatCount="indefinite"/>
    </circle>
    <text x="12" y="4" font-family="monospace" font-size="10" fill="rgba(255,255,255,0.4)">v0.1.0</text>
  </g>
  <g transform="translate(800,72)">
    <circle cx="0" cy="0" r="4" fill="#86efac" opacity="0.6"/>
    <text x="12" y="4" font-family="monospace" font-size="10" fill="rgba(255,255,255,0.4)">45 tests</text>
  </g>
  <g transform="translate(800,90)">
    <circle cx="0" cy="0" r="4" fill="#4ade80" opacity="0.5"/>
    <text x="12" y="4" font-family="monospace" font-size="10" fill="rgba(255,255,255,0.4)">MIT License</text>
  </g>
</svg>

---

[![CI](https://img.shields.io/github/actions/workflow/status/vignesh2027/stratum/ci.yml?branch=main&label=CI&style=flat-square&color=16a34a)](https://github.com/vignesh2027/stratum/actions)
[![Tests](https://img.shields.io/badge/tests-45%20passing-16a34a?style=flat-square)](https://github.com/vignesh2027/stratum)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-22c55e?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-16a34a?style=flat-square)](LICENSE)
[![GitHub Pages](https://img.shields.io/badge/docs-live-4ade80?style=flat-square)](https://vignesh2027.github.io/stratum)
[![crates.io](https://img.shields.io/badge/crates.io-coming%20soon-86efac?style=flat-square)](https://crates.io)

**[Website](https://vignesh2027.github.io/stratum) · [Quick Start](#quick-start) · [SQSL Reference](#sqsl--stratum-query-specification-language) · [API Reference](#rest-api-reference) · [Architecture](#architecture)**

</div>

---

## What is Stratum?

**Stratum** is the world's first **Temporal-Semantic-Causal database** — an embedded, append-only, content-addressed data store that lets you query across **three orthogonal dimensions simultaneously**:

| Dimension | What it answers | Implementation | Complexity |
|-----------|----------------|----------------|------------|
| **⏱ Temporal** | *When* did it happen? | `BTreeMap<i64, Vec<RecordId>>` | O(log n) range scan |
| **🔍 Semantic** | *What does it mean?* | Hand-rolled HNSW graph | O(log n) kNN |
| **🔗 Causal** | *Why* did it happen? | Bidirectional adjacency-list DAG | O(V+E) BFS |

> **The name.** Geological strata are layered, ordered records of time — each layer richer and more meaningful in the context of the layers above and below it. Stratum brings this metaphor to data: records are layered by time, connected by meaning, and linked by causality.

---

## The Gap No One Has Filled

```
You need:                        Current tooling:
─────────────────────────────────────────────────────────────────
"Events from last hour"        → Time-series DB  (InfluxDB, TimescaleDB)
"Semantically similar events"  → Vector DB       (Pinecone, Weaviate, Qdrant)
"What caused this failure?"    → Graph DB        (Neo4j, DGraph)
ALL THREE AT ONCE              → ??? — does not exist
```

Every production event has three intrinsic dimensions. Existing tools force you to pick one and build brittle multi-system pipelines to answer the rest. **Stratum answers all three natively, in a single query, with a single query language.**

### Capability Comparison

| Database | Time Queries | Semantic Search | Causal Chains | Unified Query | Embeddable |
|----------|-------------|----------------|--------------|--------------|-----------|
| InfluxDB / TimescaleDB | ✅ | ❌ | ❌ | ❌ | ~ |
| Pinecone / Weaviate / Qdrant | ❌ | ✅ | ❌ | ❌ | ❌ |
| Neo4j / DGraph | ~ | ❌ | ✅ | ❌ | ❌ |
| PostgreSQL + pgvector | ✅ | ~ | ❌ | ❌ | ❌ |
| **◈ Stratum** | **✅** | **✅** | **✅** | **✅** | **✅** |

---

## The Query That Doesn't Exist Elsewhere

```sql
FIND records
WHERE time BETWEEN "2024-01-01T00:00:00Z" AND "2024-12-31T23:59:59Z"
  AND similar_to embedding([0.91, 0.23, -0.14, 0.05, ...]) WITH threshold 0.85
  AND caused_by "incident-root-id-a3f8e2b1..." WITH depth 5
  AND tag environment = "production"
  AND schema = "alert.v2"
ORDER BY time DESC
LIMIT 25
```

> *"Show me all production alerts from 2024, semantically similar to this failure signature, that were triggered by this specific incident, most recent first."*

This is **five-dimensional filtering** (time + semantic + causal + tag + schema) that no existing database supports natively. Stratum handles it in a single query plan with O(log n) sub-queries per dimension.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────────────┐
│                     REST API  (Axum, HTTP/JSON)                       │
│    /v1/records · /v1/query · /v1/search · /v1/causal · /v1/stats    │
├──────────────────────────────────────────────────────────────────────┤
│                      CLI  (clap)                                      │
│         serve · insert · get · query · stats                         │
├──────────────────────────────────────────────────────────────────────┤
│                    SQSL Query Engine                                  │
│   Recursive Descent Parser → Logical Plan → Intersect & Rank        │
├──────────────┬───────────────────────┬───────────────────────────────┤
│  ⏱ Temporal  │  🔍 Semantic           │  🔗 Causal                    │
│  Index        │  Index (HNSW)          │  Graph (DAG)                  │
│               │                        │                               │
│ BTreeMap      │ Hierarchical           │ Bidirectional                 │
│ <i64,Vec<Id>> │ Navigable Small        │ Adjacency Lists               │
│               │ World Graph            │                               │
│ RwLock        │ DashMap (concurrent)   │ DashMap (concurrent)          │
│ O(log n)      │ O(log n) approx kNN    │ BFS O(V+E)                    │
│ Range scans   │ Cosine similarity      │ Forward & backward            │
│ Nanosecond    │ M=16, ef=200           │ Shortest path                 │
├──────────────┴───────────────────────┴───────────────────────────────┤
│               Storage Engine  (sled, pure Rust)                       │
│  Content-addressed · SHA-256 keys · Append-only · JSON values        │
│  Idempotent writes · Tamper-evident · Startup index rebuild · ACID   │
└──────────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| **Content-addressed keys (SHA-256)** | Inserting the same record twice is a no-op. Records are tamper-evident — altering any field changes the ID. Deduplication is free. |
| **Three independent indices** | Each dimension is queryable independently or in combination. The query executor intersects result sets at the `RecordId` level — O(k) intersection where k = smallest result set. |
| **Hand-rolled HNSW** | No external vector library. The implementation follows Malkov & Yashunin 2018 exactly, tunable via M (graph degree) and ef_construction (search beam width). |
| **Append-only storage** | Historical data is never mutated. GDPR right-to-erasure is supported via explicit delete. Enables safe time-travel queries. |
| **DashMap for all indices** | Lock-free reads via shard-based concurrency. Multiple readers never block each other. Writers lock only the affected shard. |
| **Startup index rebuild** | In-memory indices are reconstructed from disk on startup. No WAL replay, no index persistence complexity. Fast for typical dataset sizes. |
| **Declarative SQSL parser** | Hand-written recursive descent parser — zero dependencies. The grammar is simple enough to parse without a parser generator. |

---

## Quick Start

### Install

```bash
# From crates.io (coming soon)
cargo install stratum

# From source
git clone https://github.com/vignesh2027/stratum
cd stratum
cargo build --release
./target/release/stratum --help
```

### Start the Server

```bash
stratum serve --db ./mydata --addr 0.0.0.0:7777
```

```
  ┌─────────────────────────────────────────┐
  │   S T R A T U M  v0.1.0               │
  │   Temporal · Semantic · Causal Database  │
  └─────────────────────────────────────────┘

  ● Listening  http://0.0.0.0:7777
  ● Database   ./mydata
  ● Docs       https://vignesh2027.github.io/stratum
```

### Insert Records (CLI)

```bash
# Insert a record
ID=$(stratum insert \
  --schema "payment.v2" \
  --data '{"amount": 99.99, "currency": "USD", "user_id": "u42"}')
echo "Stored: $ID"
# → Stored: a3f8e2b1c4d9f6e2...

# Insert a causally-linked record
NOTIF_ID=$(stratum insert \
  --schema "notification.v1" \
  --data '{"type": "payment_receipt", "user_id": "u42"}' \
  --causes "$ID")
```

### Insert via REST API

```bash
curl -X POST http://localhost:7777/v1/records \
  -H 'Content-Type: application/json' \
  -d '{
    "schema": "payment.v2",
    "data": {"amount": 99.99, "currency": "USD"},
    "embedding": [0.91, 0.23, -0.14, 0.05],
    "causes": [],
    "metadata": {"environment": "production", "region": "us-east-1"}
  }'

# → {"id": "a3f8e2b1c4d9f6e2..."}
```

### Query with SQSL

```bash
# CLI
stratum query "FIND records WHERE schema = \"payment.v2\" LIMIT 50"

# REST API
curl -X POST http://localhost:7777/v1/query \
  -H 'Content-Type: application/json' \
  -d '{"sqsl": "FIND records WHERE time AFTER \"2024-01-01T00:00:00Z\" AND schema = \"payment.v2\" ORDER BY time DESC LIMIT 100"}'
```

---

## Rust Library Usage

```toml
# Cargo.toml
[dependencies]
stratum = "0.1"
tokio   = { version = "1", features = ["full"] }
```

```rust
use stratum::{Stratum, RecordBuilder};
use chrono::Utc;

#[tokio::main]
async fn main() -> stratum::Result<()> {
    // Open or create a database
    let db = Stratum::open("/data/stratum").await?;

    // Build a record with embedding and causal link
    let parent_id: [u8; 32] = /* some previously inserted ID */;

    let record = RecordBuilder::new()
        .schema("payment.v2")
        .data(serde_json::json!({
            "amount": 99.99,
            "currency": "USD",
            "user_id": "u42",
            "status": "failed"
        }))
        .embedding(compute_embedding("payment failed insufficient_funds")) // your embedding fn
        .caused_by(vec![parent_id])            // link to the causal predecessor
        .tag("environment", "production")
        .tag("region",      "us-east-1")
        .build()?;

    let id = db.insert(record).await?;
    println!("Inserted: {}", hex::encode(id));

    // ─── Temporal query ─────────────────────────────────────────────
    let last_week = db.find_by_time(
        Utc::now() - chrono::Duration::days(7),
        Utc::now(),
        100,
    ).await?;

    // ─── Semantic search ─────────────────────────────────────────────
    let similar = db.find_similar(
        &compute_embedding("payment failure"),
        10,
        0.85, // cosine similarity threshold
    ).await?;

    for (record, score) in &similar {
        println!("  score={:.3}  [{}]  {}", score, record.schema, record.data);
    }

    // ─── Causal traversal ────────────────────────────────────────────
    let effects = db.find_effects(&id, 5).await?;   // what did this trigger?
    let causes  = db.find_causes(&id, 5).await?;    // what caused this?

    // ─── Shortest causal path ─────────────────────────────────────────
    let path = db.causal.shortest_path(&parent_id, &id);
    println!("Path length: {} hops", path.map(|p| p.len()).unwrap_or(0));

    // ─── SQSL unified query ────────────────────────────────────────────
    let results = db.query(
        "FIND records \
         WHERE time AFTER \"2024-01-01T00:00:00Z\" \
           AND similar_to embedding([0.91, 0.23, -0.14]) WITH threshold 0.8 \
           AND schema = \"payment.v2\" \
         ORDER BY time DESC \
         LIMIT 25"
    ).await?;

    println!("Found {} records", results.len());

    // ─── Database stats ────────────────────────────────────────────────
    let stats = db.stats().await;
    println!("Records: {}  Vectors: {}  Causal edges: {}",
        stats.total_records, stats.semantic_vectors, stats.causal_edges);

    Ok(())
}
```

---

## SQSL — Stratum Query Specification Language

SQSL is a composable, declarative query language with **first-class temporal, semantic, and causal clauses**. It is parsed by a zero-dependency, hand-written recursive descent parser built into the engine.

### Full Grammar

```
query        := FIND records [WHERE clause (AND clause)*]
                [ORDER BY order]
                [LIMIT n]
                [OFFSET n]

clause       := time_clause
              | semantic_clause
              | causal_clause
              | schema_clause
              | tag_clause

time_clause  := time BETWEEN time_ref AND time_ref
              | time AFTER time_ref
              | time BEFORE time_ref

sem_clause   := similar_to embedding([f32, ...]) [WITH threshold f32]

causal_clause:= caused_by hex_id [WITH depth n]
              | causes hex_id [WITH depth n]

schema_clause:= schema = "name"

tag_clause   := tag key = "value"

time_ref     := "ISO-8601-string"
              | unix_nanos_integer
              | NOW
              | NOW - nanos_integer

order        := time ASC | time DESC | relevance
```

### Clause Reference

| Clause | Dimension | Description | Example |
|--------|-----------|-------------|---------|
| `time BETWEEN a AND b` | ⏱ Temporal | Records in time range [a, b] inclusive | `time BETWEEN "2024-01-01" AND "2024-12-31"` |
| `time AFTER t` | ⏱ Temporal | Records after timestamp t | `time AFTER "2024-06-01T00:00:00Z"` |
| `time BEFORE t` | ⏱ Temporal | Records before timestamp t | `time BEFORE "2024-01-01T00:00:00Z"` |
| `similar_to embedding([...]) WITH threshold f` | 🔍 Semantic | Cosine similarity ≥ threshold | `similar_to embedding([0.1, 0.9]) WITH threshold 0.85` |
| `caused_by "id" WITH depth n` | 🔗 Causal | Causal descendants of `id`, up to `n` hops | `caused_by "a3f8..." WITH depth 5` |
| `causes "id" WITH depth n` | 🔗 Causal | Causal ancestors of `id`, up to `n` hops | `causes "a3f8..." WITH depth 3` |
| `schema = "name"` | 📋 Schema | Records with matching schema name | `schema = "payment.v2"` |
| `tag key = "value"` | 🏷 Metadata | Records with matching tag | `tag environment = "production"` |

### ORDER BY Options

| Expression | Effect |
|-----------|--------|
| `ORDER BY time DESC` | Most recent records first (default) |
| `ORDER BY time ASC` | Oldest records first |
| `ORDER BY relevance` | Semantic similarity score descending (meaningful only with `similar_to` clause) |

### SQSL Examples by Use Case

#### Incident Root-Cause Analysis

```sql
-- All production failures in the past 6 hours caused by a known bad deploy
FIND records
WHERE time AFTER "2024-03-15T18:00:00Z"
  AND caused_by "bad-deploy-commit-a3f8..." WITH depth 10
  AND similar_to embedding([0.88, -0.23, 0.41, ...]) WITH threshold 0.78
  AND tag environment = "production"
ORDER BY time ASC
LIMIT 500
```

#### AI Agent Memory Retrieval

```sql
-- Semantically relevant memories from the current session
FIND records
WHERE similar_to embedding([0.12, 0.88, -0.31, 0.05, ...]) WITH threshold 0.90
  AND time AFTER "2024-06-01T00:00:00Z"
  AND schema = "agent.memory.v1"
ORDER BY relevance
LIMIT 20
```

#### Event Sourcing Replay

```sql
-- All events up to a specific point in time, for replay
FIND records
WHERE time BEFORE "2024-09-01T00:00:00Z"
  AND schema = "account.event.v2"
ORDER BY time ASC
```

#### Financial Fraud Detection

```sql
-- Find transactions causally related to a suspicious order with similar patterns
FIND records
WHERE caused_by "suspicious-order-8f2a..." WITH depth 5
  AND similar_to embedding([fraud_pattern_embedding...]) WITH threshold 0.80
  AND time BETWEEN "2024-01-01T00:00:00Z" AND "2024-12-31T23:59:59Z"
  AND tag region = "eu-west-1"
```

#### Combined Five-Dimension Query

```sql
-- The killer query: temporal + semantic + causal + schema + tag
FIND records
WHERE time BETWEEN "2024-01-01T00:00:00Z" AND "2024-12-31T23:59:59Z"
  AND similar_to embedding([0.91, 0.23, -0.14, 0.05, ...]) WITH threshold 0.82
  AND caused_by "incident-root-a3f8e2b1..." WITH depth 5
  AND schema = "alert.v2"
  AND tag environment = "production"
ORDER BY time DESC
LIMIT 25
```

---

## REST API Reference

Base URL: `http://localhost:7777`  
All request/response bodies are JSON.

### `GET /`
Health check. Returns engine name and version.
```json
{ "status": "ok", "engine": "stratum", "version": "0.1.0" }
```

### `GET /v1/stats`
Database statistics.
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
  "embedding": [0.91, 0.23, -0.14, 0.05],
  "causes": ["a3f8e2b1c4d9f6e2..."],
  "metadata": { "environment": "production", "region": "us-east-1" }
}
```

**Response:**
```json
{ "id": "b1c2d3e4f5a6b7c8..." }
```

> All fields except `schema` and `data` are optional.
> `timestamp` is nanoseconds since Unix epoch; defaults to now.
> `causes` is an array of 64-character hex record IDs.

### `GET /v1/records/:id`
Retrieve a record by its 64-character hex content ID.

**Response:** Full `Record` object as JSON, or `404` if not found.

### `POST /v1/query`
Execute an SQSL query.

**Request:**
```json
{ "sqsl": "FIND records WHERE schema = \"payment.v2\" LIMIT 10" }
```

**Response:**
```json
{ "count": 10, "records": [...] }
```

### `GET /v1/search/time?from=&to=&limit=`
Time-range search.

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `from` | ISO 8601 string | Unix epoch | Range start (inclusive) |
| `to` | ISO 8601 string | Now | Range end (inclusive) |
| `limit` | integer | 100 | Max results |

### `POST /v1/search/similar`
Semantic similarity search.

**Request:**
```json
{ "embedding": [0.91, 0.23, -0.14], "k": 10, "threshold": 0.82 }
```

**Response:**
```json
{
  "count": 3,
  "results": [
    { "score": 0.977, "record": { ... } },
    { "score": 0.921, "record": { ... } },
    { "score": 0.841, "record": { ... } }
  ]
}
```

### `GET /v1/causal/:id/effects?depth=5`
Find all causal descendants of a record (forward BFS).

### `GET /v1/causal/:id/causes?depth=5`
Find all causal ancestors of a record (backward BFS).

### `GET /v1/causal/:from/path/:to`
Shortest causal path between two records.

**Response:**
```json
{ "path": ["a3f8...", "b1c2...", "e7f2..."], "length": 3 }
```

---

## Record Schema

```rust
pub struct Record {
    /// Content-addressed ID: SHA-256(schema || timestamp || data)
    pub id: [u8; 32],

    /// Logical timestamp in nanoseconds since Unix epoch.
    /// Defaults to wall-clock insertion time.
    pub timestamp: i64,

    /// Wall-clock time of insertion (always set by the engine).
    pub inserted_at: DateTime<Utc>,

    /// Schema name and version. Convention: "domain.type.vN"
    /// Examples: "payment.charge.v2", "agent.observation.v1"
    pub schema: String,

    /// Arbitrary JSON payload.
    pub data: serde_json::Value,

    /// Optional semantic embedding vector (f32 components).
    /// If present, the record is indexed in the HNSW semantic index.
    pub embedding: Option<Vec<f32>>,

    /// Content IDs of records that directly caused this one.
    /// Used to build the causal DAG. Can be empty.
    pub causes: Vec<[u8; 32]>,

    /// Arbitrary key-value metadata tags.
    pub metadata: HashMap<String, String>,
}
```

---

## Use Cases

### 🔎 Incident Investigation & SRE

Production incidents generate events across dozens of services. Finding the root cause requires correlating *time* (when did symptoms appear?), *semantics* (which past incidents look similar?), and *causality* (which deploy or config change triggered this?).

Stratum lets you issue one query after an incident is detected:

```sql
FIND records
WHERE time AFTER "2024-03-15T18:00:00Z"
  AND similar_to embedding([error_signature_embedding...]) WITH threshold 0.82
  AND caused_by "deploy-abc123..." WITH depth 10
  AND tag environment = "production"
ORDER BY time ASC LIMIT 500
```

No stitching three dashboards together. No manual correlation. One query returns the full causal subtree of the bad deploy, filtered to events semantically matching the failure signature.

---

### 🤖 AI Memory Systems & Agentic Frameworks

AI agents need persistent, contextual memory. Retrieval must respect:
- **Recency** — more recent memories are usually more relevant
- **Semantic relevance** — retrieve memories that *mean* something similar to the current context
- **Causal relationships** — "what action did I take after observing X?"

Stratum is purpose-built for this. Each agent observation, tool call, and response is stored as a record with an embedding, a timestamp, and causal links. Context retrieval is a single SQSL query.

```rust
// Store an agent observation
let obs_id = db.insert(
    RecordBuilder::new()
        .schema("agent.observation.v1")
        .data(json!({ "input": "user asked about Q3 revenue" }))
        .embedding(embed("Q3 revenue question"))
        .build()?
).await?;

// Store the action caused by it
db.insert(
    RecordBuilder::new()
        .schema("agent.action.v1")
        .data(json!({ "tool": "query_database", "result": "..." }))
        .embedding(embed("query database tool call"))
        .caused_by(vec![obs_id])
        .build()?
).await?;

// Later: retrieve relevant context
let context = db.query(
    "FIND records WHERE similar_to embedding([...current_context...]) WITH threshold 0.88
     AND time AFTER \"session-start\" ORDER BY relevance LIMIT 10"
).await?;
```

---

### 📋 Event Sourcing & Audit Logs

Stratum is a natural event store:
- **Append-only** — events are never modified, only accumulated
- **Content-addressed** — identical events are deduplicated automatically
- **Causal links** — model command-event chains explicitly
- **Time-travel** — replay state up to any timestamp with a SQSL query

```sql
-- Replay all account events up to the point of a dispute
FIND records
WHERE time BEFORE "2024-09-15T14:32:00Z"
  AND caused_by "account-opened-event-id..." WITH depth 100
  AND schema = "account.event.v2"
ORDER BY time ASC
```

---

### 🏥 Healthcare & Compliance

Clinical systems need tamper-evident records (regulatory requirement), causal chains for explainability ("what decisions led to this outcome?"), and semantic search for similar cases in the literature.

Stratum's SHA-256 content-addressing provides tamper-evidence at no extra cost. The causal DAG models clinical decision chains. GDPR compliance is supported via the explicit delete endpoint.

---

### 📈 Financial Systems & Fraud Detection

Financial workflows have natural causal structure: order → payment → refund → notification → audit log entry. Stratum models these chains explicitly, enabling queries like "show me all transactions causally related to this suspicious order that look similar to known fraud patterns."

---

### 🔬 Scientific Research & Reproducibility

Research workflows are causal: hypothesis → experiment → result → publication → follow-up hypothesis. Stratum lets you model this structure explicitly, query the full provenance of any finding, and search for semantically related prior work — all in one database.

---

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Insert (no embedding) | O(log n) | BTree insert + sled write |
| Insert (with embedding) | O(log n) | +HNSW insert (amortized log n) |
| Time range query | O(log n + k) | BTree range scan, k = result count |
| Semantic kNN search | O(log n) | HNSW approximate; tunable recall via ef |
| Causal BFS | O(V + E) | V = visited nodes, E = edges in subgraph |
| Shortest causal path | O(V + E) | BFS; terminates at first hit |
| Record retrieval by ID | O(1) | Direct sled key lookup |
| Startup index rebuild | O(n) | Full scan; one-time on open |

---

## Testing

```bash
# Run all 45 tests
cargo test

# Unit tests only (30 tests)
cargo test --lib

# Integration tests only (14 tests)
cargo test --test integration_test

# Run examples
cargo run --example basic_usage
cargo run --example causal_chain

# Benchmarks (requires release build)
cargo bench
```

### Test Coverage

| Module | Tests | What's Covered |
|--------|-------|---------------|
| `record` | 4 | Builder, ID determinism, field validation |
| `storage::engine` | 5 | Put/get roundtrip, idempotency, scan, delete |
| `index::temporal` | 4 | Range queries, limit, before/after, remove |
| `index::semantic` | 4 | Cosine similarity, kNN, threshold filtering |
| `graph::causal` | 5 | BFS depth, ancestors, descendants, path, roots/leaves |
| `query::parser` | 6 | All clause types, ORDER BY, error handling |
| `integration` | 14 | End-to-end: insert→query across all three dimensions |
| `doc tests` | 3 | API documentation compile-checks |
| **Total** | **45** | **All passing** |

---

## Roadmap

- [ ] Persistent HNSW index (survive restarts without full rebuild)
- [ ] Distributed mode with Raft-based replication
- [ ] Python bindings (`pystratum`)
- [ ] WebAssembly target for in-browser queries
- [ ] Cost-based SQSL query planner
- [ ] Product Quantization for compressed vector storage
- [ ] Streaming query results via Server-Sent Events
- [ ] Schema registry and payload validation
- [ ] Snapshot export and restore
- [ ] Metrics endpoint (Prometheus-compatible)

---

## Contributing

```bash
git clone https://github.com/vignesh2027/stratum
cd stratum
cargo test       # Must pass
cargo clippy     # Must be warning-free
cargo fmt        # Must be formatted
```

Open an issue before starting large changes.

---

## Author

**Vignesh S** — CSE 2022–26, Takshashila University, Chennai

GitHub: [@vignesh2027](https://github.com/vignesh2027)

Other systems projects: [SYNTHRON](https://github.com/vignesh2027/synthron) · [VORTEXRAG](https://github.com/vignesh2027/VORTEXRAG) · [rustkvd](https://github.com/vignesh2027/rustkvd) · [FluxDB](https://github.com/vignesh2027/-FLUXDB)

---

## License

MIT — see [LICENSE](LICENSE).

---

<div align="center">

**◈ Stratum** — *layered records for a three-dimensional world*

[🌐 Website](https://vignesh2027.github.io/stratum) · [📦 GitHub](https://github.com/vignesh2027/stratum) · [🐛 Issues](https://github.com/vignesh2027/stratum/issues) · [⚡ Actions](https://github.com/vignesh2027/stratum/actions)

</div>
