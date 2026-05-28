use crate::error::{Result, StratumError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// A 32-byte content-addressed record ID (SHA-256).
pub type RecordId = [u8; 32];

/// An immutable, content-addressed record in Stratum.
///
/// Every field is frozen at write time. The `id` is the SHA-256 hash of
/// (schema + data + timestamp), making records tamper-evident by construction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    /// Content-addressed identifier (SHA-256 of schema+data+timestamp).
    pub id: RecordId,

    /// Logical timestamp in nanoseconds since Unix epoch.
    pub timestamp: i64,

    /// Wall-clock time of insertion (may differ from logical timestamp).
    pub inserted_at: DateTime<Utc>,

    /// Schema name and version, e.g. "payment.v2" or "sensor_reading.v1".
    pub schema: String,

    /// Arbitrary payload — typically JSON, but any bytes are valid.
    pub data: serde_json::Value,

    /// Optional semantic embedding vector for similarity search.
    /// If provided, the record will be indexed by the SemanticIndex.
    pub embedding: Option<Vec<f32>>,

    /// Causal predecessors: IDs of records that directly caused this one.
    /// Used to construct the causal DAG.
    pub causes: Vec<RecordId>,

    /// Arbitrary key-value metadata (tags, source system, etc.).
    pub metadata: HashMap<String, String>,
}

impl Record {
    /// Compute the content-addressed ID for a record given its core fields.
    pub fn compute_id(schema: &str, data: &serde_json::Value, timestamp: i64) -> RecordId {
        let mut hasher = Sha256::new();
        hasher.update(schema.as_bytes());
        hasher.update(timestamp.to_le_bytes());
        hasher.update(data.to_string().as_bytes());
        hasher.finalize().into()
    }

    /// Human-readable hex representation of the record ID.
    pub fn id_hex(&self) -> String {
        hex::encode(self.id)
    }

    /// Logical timestamp as a UTC DateTime.
    pub fn logical_time(&self) -> Option<DateTime<Utc>> {
        use chrono::TimeZone;
        let secs = self.timestamp / 1_000_000_000;
        let nanos = (self.timestamp % 1_000_000_000) as u32;
        Utc.timestamp_opt(secs, nanos).single()
    }
}

/// Builder for constructing [`Record`] values ergonomically.
pub struct RecordBuilder {
    timestamp: Option<i64>,
    schema: String,
    data: serde_json::Value,
    embedding: Option<Vec<f32>>,
    causes: Vec<RecordId>,
    metadata: HashMap<String, String>,
}

impl Default for RecordBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordBuilder {
    pub fn new() -> Self {
        Self {
            timestamp: None,
            schema: "default.v1".to_string(),
            data: serde_json::Value::Null,
            embedding: None,
            causes: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set the logical timestamp (nanoseconds since Unix epoch).
    /// Defaults to now if not set.
    pub fn timestamp(mut self, ts: i64) -> Self {
        self.timestamp = Some(ts);
        self
    }

    /// Set the logical timestamp from a DateTime.
    pub fn at(mut self, dt: DateTime<Utc>) -> Self {
        self.timestamp = dt.timestamp_nanos_opt();
        self
    }

    /// Set the schema name.
    pub fn schema(mut self, s: impl Into<String>) -> Self {
        self.schema = s.into();
        self
    }

    /// Set the JSON payload.
    pub fn data(mut self, d: serde_json::Value) -> Self {
        self.data = d;
        self
    }

    /// Attach a semantic embedding vector.
    pub fn embedding(mut self, emb: Vec<f32>) -> Self {
        self.embedding = Some(emb);
        self
    }

    /// Declare causal predecessors.
    pub fn caused_by(mut self, causes: Vec<RecordId>) -> Self {
        self.causes = causes;
        self
    }

    /// Add a single metadata tag.
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Finalize and build the Record.
    pub fn build(self) -> Result<Record> {
        if self.schema.is_empty() {
            return Err(StratumError::InvalidRecord(
                "schema must not be empty".into(),
            ));
        }
        let timestamp = self
            .timestamp
            .unwrap_or_else(|| Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let id = Record::compute_id(&self.schema, &self.data, timestamp);
        Ok(Record {
            id,
            timestamp,
            inserted_at: Utc::now(),
            schema: self.schema,
            data: self.data,
            embedding: self.embedding,
            causes: self.causes,
            metadata: self.metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_produces_deterministic_id() {
        let r1 = RecordBuilder::new()
            .schema("test.v1")
            .data(serde_json::json!({"x": 1}))
            .timestamp(1_000_000)
            .build()
            .unwrap();
        let r2 = RecordBuilder::new()
            .schema("test.v1")
            .data(serde_json::json!({"x": 1}))
            .timestamp(1_000_000)
            .build()
            .unwrap();
        assert_eq!(r1.id, r2.id);
    }

    #[test]
    fn different_data_produces_different_id() {
        let r1 = RecordBuilder::new()
            .schema("test.v1")
            .data(serde_json::json!({"x": 1}))
            .timestamp(1_000_000)
            .build()
            .unwrap();
        let r2 = RecordBuilder::new()
            .schema("test.v1")
            .data(serde_json::json!({"x": 2}))
            .timestamp(1_000_000)
            .build()
            .unwrap();
        assert_ne!(r1.id, r2.id);
    }

    #[test]
    fn id_hex_is_64_chars() {
        let r = RecordBuilder::new().build().unwrap();
        assert_eq!(r.id_hex().len(), 64);
    }

    #[test]
    fn builder_validates_empty_schema() {
        let result = RecordBuilder::new().schema("").build();
        assert!(result.is_err());
    }
}
