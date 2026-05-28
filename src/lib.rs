pub mod api;
pub mod error;
pub mod graph;
pub mod index;
pub mod query;
pub mod record;
pub mod storage;

pub use error::{Result, StratumError};
pub use record::{Record, RecordBuilder, RecordId};
pub use storage::StorageEngine;

use std::path::Path;
use std::sync::Arc;

/// The primary Stratum database handle.
///
/// Stratum is an embedded temporal-semantic-causal database.
/// Every record is immutable, content-addressed, and queryable across
/// three orthogonal dimensions: time, semantic similarity, and causal lineage.
///
/// # Example
/// ```rust,no_run
/// use stratum::{Stratum, RecordBuilder};
///
/// #[tokio::main]
/// async fn main() -> stratum::Result<()> {
///     let db = Stratum::open("/tmp/mydb").await?;
///
///     let record = RecordBuilder::new()
///         .schema("payment.v1")
///         .data(serde_json::json!({"amount": 99.99, "currency": "USD"}))
///         .embedding(vec![0.1, 0.2, 0.3, 0.4])
///         .build()?;
///
///     let id = db.insert(record).await?;
///     println!("Stored: {}", hex::encode(id));
///     Ok(())
/// }
/// ```
pub struct Stratum {
    pub(crate) storage: Arc<StorageEngine>,
    pub(crate) temporal: Arc<index::TemporalIndex>,
    pub(crate) semantic: Arc<index::SemanticIndex>,
    pub causal: Arc<graph::CausalGraph>,
}

impl Stratum {
    /// Open (or create) a Stratum database at the given path.
    pub async fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let storage = Arc::new(StorageEngine::open(path.as_ref())?);
        let temporal = Arc::new(index::TemporalIndex::new());
        let semantic = Arc::new(index::SemanticIndex::new(128));
        let causal = Arc::new(graph::CausalGraph::new());

        let db = Self {
            storage,
            temporal,
            semantic,
            causal,
        };
        db.rebuild_indices().await?;
        Ok(db)
    }

    /// Open an in-memory Stratum database (for testing/ephemeral use).
    pub async fn open_memory() -> Result<Self> {
        let storage = Arc::new(StorageEngine::open_memory()?);
        let temporal = Arc::new(index::TemporalIndex::new());
        let semantic = Arc::new(index::SemanticIndex::new(128));
        let causal = Arc::new(graph::CausalGraph::new());
        Ok(Self {
            storage,
            temporal,
            semantic,
            causal,
        })
    }

    /// Insert a record into Stratum. Returns the content-addressed ID.
    pub async fn insert(&self, record: Record) -> Result<RecordId> {
        let id = record.id;
        let ts = record.timestamp;
        let embedding = record.embedding.clone();
        let causes = record.causes.clone();

        self.storage.put(&record)?;
        self.temporal.insert(ts, id);

        if let Some(emb) = &embedding {
            self.semantic.insert(id, emb);
        }
        for cause in &causes {
            self.causal.add_edge(*cause, id);
        }
        self.causal.add_node(id);

        Ok(id)
    }

    /// Retrieve a record by its content-addressed ID.
    pub async fn get(&self, id: &RecordId) -> Result<Option<Record>> {
        self.storage.get(id)
    }

    /// Execute an SQSL query string.
    pub async fn query(&self, sqsl: &str) -> Result<Vec<Record>> {
        let plan = query::parse(sqsl)?;
        query::execute(plan, self).await
    }

    /// Find all records in a time range.
    pub async fn find_by_time(
        &self,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
        limit: usize,
    ) -> Result<Vec<Record>> {
        let ids = self.temporal.range(
            from.timestamp_nanos_opt().unwrap_or(0),
            to.timestamp_nanos_opt().unwrap_or(i64::MAX),
            limit,
        );
        self.fetch_records(&ids).await
    }

    /// Find the K nearest records by semantic embedding similarity.
    pub async fn find_similar(
        &self,
        embedding: &[f32],
        k: usize,
        threshold: f32,
    ) -> Result<Vec<(Record, f32)>> {
        let hits = self.semantic.search(embedding, k, threshold);
        let mut results = Vec::new();
        for (id, score) in hits {
            if let Some(rec) = self.storage.get(&id)? {
                results.push((rec, score));
            }
        }
        Ok(results)
    }

    /// Traverse the causal chain forward from a root record.
    pub async fn find_effects(&self, root: &RecordId, depth: usize) -> Result<Vec<Record>> {
        let ids = self.causal.descendants(root, depth);
        self.fetch_records(&ids).await
    }

    /// Traverse the causal chain backward to find all ancestors.
    pub async fn find_causes(&self, id: &RecordId, depth: usize) -> Result<Vec<Record>> {
        let ids = self.causal.ancestors(id, depth);
        self.fetch_records(&ids).await
    }

    /// Return database statistics.
    pub async fn stats(&self) -> DbStats {
        DbStats {
            total_records: self.storage.count(),
            temporal_entries: self.temporal.len(),
            semantic_vectors: self.semantic.len(),
            causal_edges: self.causal.edge_count(),
        }
    }

    async fn fetch_records(&self, ids: &[RecordId]) -> Result<Vec<Record>> {
        let mut out = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(r) = self.storage.get(id)? {
                out.push(r);
            }
        }
        Ok(out)
    }

    async fn rebuild_indices(&self) -> Result<()> {
        for record in self.storage.scan_all()? {
            let ts = record.timestamp;
            let id = record.id;
            self.temporal.insert(ts, id);
            if let Some(emb) = &record.embedding {
                self.semantic.insert(id, emb);
            }
            self.causal.add_node(id);
            for cause in &record.causes {
                self.causal.add_edge(*cause, id);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DbStats {
    pub total_records: usize,
    pub temporal_entries: usize,
    pub semantic_vectors: usize,
    pub causal_edges: usize,
}
