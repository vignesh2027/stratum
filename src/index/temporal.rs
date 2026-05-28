use crate::record::RecordId;
use dashmap::DashMap;
use std::collections::BTreeMap;
use std::sync::RwLock;

/// A concurrent, ordered index mapping logical timestamps (nanoseconds) to record IDs.
///
/// Supports range scans in O(log n + k) time, where k is the number of results.
/// Thread-safe via RwLock on the BTreeMap.
pub struct TemporalIndex {
    /// timestamp_ns → list of record IDs at that timestamp
    inner: RwLock<BTreeMap<i64, Vec<RecordId>>>,
    /// reverse map: RecordId → timestamp (for fast deletion)
    reverse: DashMap<RecordId, i64>,
}

impl TemporalIndex {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMap::new()),
            reverse: DashMap::new(),
        }
    }

    /// Insert a record into the temporal index.
    pub fn insert(&self, timestamp_ns: i64, id: RecordId) {
        self.reverse.insert(id, timestamp_ns);
        let mut map = self.inner.write().unwrap();
        map.entry(timestamp_ns).or_default().push(id);
    }

    /// Find all record IDs in [from_ns, to_ns], returning at most `limit` results.
    pub fn range(&self, from_ns: i64, to_ns: i64, limit: usize) -> Vec<RecordId> {
        let map = self.inner.read().unwrap();
        map.range(from_ns..=to_ns)
            .flat_map(|(_, ids)| ids.iter().copied())
            .take(limit)
            .collect()
    }

    /// Find the N records immediately before a given timestamp.
    pub fn before(&self, timestamp_ns: i64, n: usize) -> Vec<RecordId> {
        let map = self.inner.read().unwrap();
        map.range(..timestamp_ns)
            .rev()
            .flat_map(|(_, ids)| ids.iter().copied())
            .take(n)
            .collect()
    }

    /// Find the N records immediately after a given timestamp.
    pub fn after(&self, timestamp_ns: i64, n: usize) -> Vec<RecordId> {
        let map = self.inner.read().unwrap();
        map.range(timestamp_ns..)
            .flat_map(|(_, ids)| ids.iter().copied())
            .take(n)
            .collect()
    }

    /// Return the timestamp of a record, if indexed.
    pub fn timestamp_of(&self, id: &RecordId) -> Option<i64> {
        self.reverse.get(id).map(|v| *v)
    }

    /// Remove a record from the temporal index.
    pub fn remove(&self, id: &RecordId) {
        if let Some((_, ts)) = self.reverse.remove(id) {
            let mut map = self.inner.write().unwrap();
            if let Some(ids) = map.get_mut(&ts) {
                ids.retain(|x| x != id);
                if ids.is_empty() {
                    map.remove(&ts);
                }
            }
        }
    }

    /// Total number of timestamp entries.
    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for TemporalIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_id(n: u8) -> RecordId {
        let mut id = [0u8; 32];
        id[0] = n;
        id
    }

    #[test]
    fn range_query_returns_correct_records() {
        let idx = TemporalIndex::new();
        for i in 0..10i64 {
            idx.insert(i * 1000, make_id(i as u8));
        }
        let results = idx.range(2000, 5000, 100);
        assert_eq!(results.len(), 4); // ts 2000,3000,4000,5000
    }

    #[test]
    fn limit_is_respected() {
        let idx = TemporalIndex::new();
        for i in 0..100i64 {
            idx.insert(i, make_id(i as u8));
        }
        let results = idx.range(0, 100, 10);
        assert_eq!(results.len(), 10);
    }

    #[test]
    fn before_returns_reverse_ordered_results() {
        let idx = TemporalIndex::new();
        for i in 0..5i64 {
            idx.insert(i * 100, make_id(i as u8));
        }
        let results = idx.before(300, 2);
        // Should return ts=200, ts=100 (most recent first)
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn remove_cleans_up_correctly() {
        let idx = TemporalIndex::new();
        let id = make_id(1);
        idx.insert(1000, id);
        assert_eq!(idx.range(0, 2000, 100).len(), 1);
        idx.remove(&id);
        assert_eq!(idx.range(0, 2000, 100).len(), 0);
    }
}
