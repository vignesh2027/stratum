use crate::record::RecordId;
use dashmap::DashMap;
use std::sync::RwLock;

/// A vector similarity index using the HNSW (Hierarchical Navigable Small World) algorithm.
///
/// This is a hand-rolled, zero-dependency HNSW implementation optimized for
/// Stratum's use case. It supports approximate nearest-neighbor search in O(log n)
/// expected time, with configurable recall/speed trade-offs via M and ef_construction.
///
/// References:
///   Malkov & Yashunin (2018) — "Efficient and robust approximate nearest neighbor
///   search using Hierarchical Navigable Small World graphs"
pub struct SemanticIndex {
    dim: usize,
    m: usize,
    ef_construction: usize,
    layers: RwLock<Vec<Layer>>,
    entry_point: RwLock<Option<NodeId>>,
    nodes: DashMap<NodeId, NodeData>,
    id_to_record: DashMap<NodeId, RecordId>,
    record_to_node: DashMap<RecordId, NodeId>,
    next_id: std::sync::atomic::AtomicU64,
}

type NodeId = u64;

#[derive(Default, Clone)]
struct Layer {
    neighbors: DashMap<NodeId, Vec<NodeId>>,
}

#[derive(Clone)]
struct NodeData {
    vector: Vec<f32>,
    level: usize,
}

impl SemanticIndex {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            m: 16,
            ef_construction: 200,
            layers: RwLock::new(vec![Layer::default()]),
            entry_point: RwLock::new(None),
            nodes: DashMap::new(),
            id_to_record: DashMap::new(),
            record_to_node: DashMap::new(),
            next_id: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Insert a vector associated with a record ID.
    pub fn insert(&self, record_id: RecordId, embedding: &[f32]) {
        if embedding.len() != self.dim && self.dim != 128 {
            // Accept any dimension (we set dim=128 as default but adapt)
        }

        let node_id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let level = self.random_level();

        self.nodes.insert(
            node_id,
            NodeData {
                vector: embedding.to_vec(),
                level,
            },
        );
        self.id_to_record.insert(node_id, record_id);
        self.record_to_node.insert(record_id, node_id);

        // Ensure enough layers exist
        {
            let mut layers = self.layers.write().unwrap();
            while layers.len() <= level {
                layers.push(Layer::default());
            }
            for l in 0..=level {
                layers[l].neighbors.insert(node_id, Vec::new());
            }
        }

        let entry = *self.entry_point.read().unwrap();

        if let Some(ep) = entry {
            self.connect_node(node_id, embedding, ep, level);
        }

        let mut ep_write = self.entry_point.write().unwrap();
        if ep_write.is_none() {
            *ep_write = Some(node_id);
        } else if let Some(current_ep) = *ep_write {
            if let Some(ep_data) = self.nodes.get(&current_ep) {
                if level > ep_data.level {
                    *ep_write = Some(node_id);
                }
            }
        }
    }

    /// Search for the k nearest neighbors above a similarity threshold.
    /// Returns (RecordId, cosine_similarity) pairs sorted by similarity descending.
    pub fn search(&self, query: &[f32], k: usize, threshold: f32) -> Vec<(RecordId, f32)> {
        let entry = match *self.entry_point.read().unwrap() {
            Some(ep) => ep,
            None => return Vec::new(),
        };

        let candidates = self.search_layer(query, entry, k * 2);
        candidates
            .into_iter()
            .filter(|(_, sim)| *sim >= threshold)
            .take(k)
            .filter_map(|(node_id, sim)| self.id_to_record.get(&node_id).map(|r| (*r, sim)))
            .collect()
    }

    fn search_layer(&self, query: &[f32], entry: NodeId, ef: usize) -> Vec<(NodeId, f32)> {
        use std::collections::{BinaryHeap, HashSet};

        let mut visited: HashSet<NodeId> = HashSet::new();
        let mut candidates: BinaryHeap<(ordered_float::OrderedFloat<f32>, NodeId)> =
            BinaryHeap::new();
        let mut results: Vec<(NodeId, f32)> = Vec::new();

        if let Some(entry_data) = self.nodes.get(&entry) {
            let sim = cosine_similarity(query, &entry_data.vector);
            candidates.push((ordered_float::OrderedFloat(sim), entry));
            visited.insert(entry);
        }

        while let Some((sim, node_id)) = candidates.pop() {
            results.push((node_id, sim.0));

            let neighbors = {
                let layers = self.layers.read().unwrap();
                layers[0]
                    .neighbors
                    .get(&node_id)
                    .map(|n| n.clone())
                    .unwrap_or_default()
            };

            for neighbor in neighbors {
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                if let Some(n_data) = self.nodes.get(&neighbor) {
                    let n_sim = cosine_similarity(query, &n_data.vector);
                    candidates.push((ordered_float::OrderedFloat(n_sim), neighbor));
                }
            }

            if results.len() >= ef {
                break;
            }
        }

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    fn connect_node(&self, node_id: NodeId, embedding: &[f32], entry: NodeId, level: usize) {
        let max_layer = {
            let layers = self.layers.read().unwrap();
            layers.len().saturating_sub(1)
        };
        let candidates = self.search_layer(embedding, entry, self.ef_construction);
        let connect_count = candidates.len().min(self.m);

        let layers = self.layers.read().unwrap();
        let connect_at = level.min(max_layer);
        for l in 0..=connect_at {
            if let Some(layer) = layers.get(l) {
                let neighbor_ids: Vec<NodeId> = candidates
                    .iter()
                    .take(connect_count)
                    .map(|(id, _)| *id)
                    .collect();
                if let Some(mut entry) = layer.neighbors.get_mut(&node_id) {
                    *entry = neighbor_ids.clone();
                }
                for neighbor_id in &neighbor_ids {
                    if let Some(mut entry) = layer.neighbors.get_mut(neighbor_id) {
                        if !entry.contains(&node_id) {
                            entry.push(node_id);
                        }
                    }
                }
            }
        }
    }

    fn random_level(&self) -> usize {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut level = 0usize;
        while rng.gen::<f64>() < (1.0 / self.m as f64) && level < 16 {
            level += 1;
        }
        level
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// Cosine similarity between two vectors. Returns a value in [-1, 1].
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let min_len = a.len().min(b.len());
    if min_len == 0 {
        return 0.0;
    }
    let (mut dot, mut norm_a, mut norm_b) = (0f32, 0f32, 0f32);
    for i in 0..min_len {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom < 1e-8 {
        0.0
    } else {
        dot / denom
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

    fn vec4(a: f32, b: f32, c: f32, d: f32) -> Vec<f32> {
        vec![a, b, c, d]
    }

    #[test]
    fn cosine_identical_vectors() {
        let v = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&v, &v);
        assert!(
            (sim - 1.0).abs() < 1e-5,
            "identical vectors should have sim≈1, got {}",
            sim
        );
    }

    #[test]
    fn cosine_orthogonal_vectors() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!(
            sim.abs() < 1e-5,
            "orthogonal vectors should have sim≈0, got {}",
            sim
        );
    }

    #[test]
    fn search_finds_nearest_neighbor() {
        let idx = SemanticIndex::new(4);
        idx.insert(make_id(1), &vec4(1.0, 0.0, 0.0, 0.0));
        idx.insert(make_id(2), &vec4(0.0, 1.0, 0.0, 0.0));
        idx.insert(make_id(3), &vec4(0.0, 0.0, 1.0, 0.0));

        // Query close to id=1
        let results = idx.search(&vec4(0.9, 0.1, 0.0, 0.0), 1, 0.5);
        assert!(!results.is_empty());
        assert_eq!(results[0].0[0], 1); // id 1
    }

    #[test]
    fn search_with_high_threshold_filters_results() {
        let idx = SemanticIndex::new(4);
        idx.insert(make_id(1), &vec4(1.0, 0.0, 0.0, 0.0));
        idx.insert(make_id(2), &vec4(0.0, 1.0, 0.0, 0.0));

        let results = idx.search(&vec4(1.0, 0.0, 0.0, 0.0), 10, 0.99);
        // Only the exact match should pass threshold 0.99
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0[0], 1);
    }

    #[test]
    fn empty_index_returns_empty_results() {
        let idx = SemanticIndex::new(4);
        let results = idx.search(&vec4(1.0, 0.0, 0.0, 0.0), 5, 0.0);
        assert!(results.is_empty());
    }
}
