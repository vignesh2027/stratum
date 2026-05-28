use crate::error::Result;
use crate::record::{Record, RecordId};
use std::path::Path;

const RECORDS_TREE: &str = "records";

/// Append-only, content-addressed storage engine backed by sled.
///
/// Records are keyed by their 32-byte SHA-256 content hash.
/// Writes are idempotent — inserting the same record twice is a no-op.
pub struct StorageEngine {
    db: sled::Db,
    records: sled::Tree,
}

impl StorageEngine {
    pub fn open(path: &Path) -> Result<Self> {
        let db = sled::Config::new().path(path).open()?;
        let records = db.open_tree(RECORDS_TREE)?;
        Ok(Self { db, records })
    }

    pub fn open_memory() -> Result<Self> {
        let db = sled::Config::new().temporary(true).open()?;
        let records = db.open_tree(RECORDS_TREE)?;
        Ok(Self { db, records })
    }

    /// Persist a record. Returns true if it was newly inserted, false if already existed.
    pub fn put(&self, record: &Record) -> Result<bool> {
        let key = record.id;
        if self.records.contains_key(key)? {
            return Ok(false);
        }
        let encoded = serde_json::to_vec(record)?;
        self.records.insert(key, encoded)?;
        Ok(true)
    }

    /// Retrieve a record by its content-addressed ID.
    pub fn get(&self, id: &RecordId) -> Result<Option<Record>> {
        match self.records.get(id)? {
            Some(bytes) => {
                let record: Record = serde_json::from_slice(&bytes)?;
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    /// Delete a record. Normally Akasha is append-only, but this is provided
    /// for compliance (GDPR right-to-erasure) scenarios.
    pub fn delete(&self, id: &RecordId) -> Result<bool> {
        Ok(self.records.remove(id)?.is_some())
    }

    /// Scan all records (used for index rebuilding on startup).
    pub fn scan_all(&self) -> Result<impl Iterator<Item = Record>> {
        let iter = self.records.iter();
        let records: Vec<Record> = iter
            .filter_map(|res| res.ok())
            .filter_map(|(_, v)| serde_json::from_slice::<Record>(&v).ok())
            .collect();
        Ok(records.into_iter())
    }

    /// Total number of records stored.
    pub fn count(&self) -> usize {
        self.records.len()
    }

    /// Flush all pending writes to disk.
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Size of the database on disk in bytes.
    pub fn size_on_disk(&self) -> Result<u64> {
        Ok(self.db.size_on_disk()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::RecordBuilder;

    fn make_engine() -> StorageEngine {
        StorageEngine::open_memory().unwrap()
    }

    #[test]
    fn put_and_get_roundtrip() {
        let engine = make_engine();
        let record = RecordBuilder::new()
            .schema("test.v1")
            .data(serde_json::json!({"hello": "world"}))
            .build()
            .unwrap();
        let id = record.id;
        engine.put(&record).unwrap();
        let retrieved = engine.get(&id).unwrap().unwrap();
        assert_eq!(retrieved.id, id);
        assert_eq!(retrieved.schema, "test.v1");
    }

    #[test]
    fn get_missing_returns_none() {
        let engine = make_engine();
        let id = [0u8; 32];
        assert!(engine.get(&id).unwrap().is_none());
    }

    #[test]
    fn put_is_idempotent() {
        let engine = make_engine();
        let record = RecordBuilder::new().build().unwrap();
        let first = engine.put(&record).unwrap();
        let second = engine.put(&record).unwrap();
        assert!(first);
        assert!(!second);
        assert_eq!(engine.count(), 1);
    }

    #[test]
    fn scan_all_returns_all_records() {
        let engine = make_engine();
        for i in 0..10u64 {
            let record = RecordBuilder::new()
                .schema("test.v1")
                .data(serde_json::json!({"i": i}))
                .timestamp(i as i64)
                .build()
                .unwrap();
            engine.put(&record).unwrap();
        }
        let all: Vec<_> = engine.scan_all().unwrap().collect();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn delete_removes_record() {
        let engine = make_engine();
        let record = RecordBuilder::new().build().unwrap();
        let id = record.id;
        engine.put(&record).unwrap();
        assert!(engine.delete(&id).unwrap());
        assert!(engine.get(&id).unwrap().is_none());
        assert!(!engine.delete(&id).unwrap());
    }
}
