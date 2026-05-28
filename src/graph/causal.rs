use crate::record::RecordId;
use dashmap::DashMap;
use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A concurrent, append-only causal DAG (Directed Acyclic Graph).
///
/// Edges represent causal relationships: an edge A → B means "A caused B."
/// The graph is stored as two adjacency maps (forward and reverse) for
/// efficient traversal in both directions.
///
/// Thread safety: uses DashMap for concurrent writes and reads.
pub struct CausalGraph {
    /// forward edges: cause → effects
    forward: DashMap<RecordId, Vec<RecordId>>,
    /// reverse edges: effect → causes
    reverse: DashMap<RecordId, Vec<RecordId>>,
    edge_count: AtomicUsize,
}

impl CausalGraph {
    pub fn new() -> Self {
        Self {
            forward: DashMap::new(),
            reverse: DashMap::new(),
            edge_count: AtomicUsize::new(0),
        }
    }

    /// Register a node without any edges (so isolated records appear in the graph).
    pub fn add_node(&self, id: RecordId) {
        self.forward.entry(id).or_default();
        self.reverse.entry(id).or_default();
    }

    /// Add a causal edge: `cause` → `effect`.
    pub fn add_edge(&self, cause: RecordId, effect: RecordId) {
        self.forward.entry(cause).or_default().push(effect);
        self.reverse.entry(effect).or_default().push(cause);
        self.edge_count.fetch_add(1, Ordering::Relaxed);
    }

    /// BFS forward: return all descendants of `root` up to `max_depth` hops.
    pub fn descendants(&self, root: &RecordId, max_depth: usize) -> Vec<RecordId> {
        self.bfs(root, max_depth, true)
    }

    /// BFS backward: return all ancestors of `node` up to `max_depth` hops.
    pub fn ancestors(&self, node: &RecordId, max_depth: usize) -> Vec<RecordId> {
        self.bfs(node, max_depth, false)
    }

    /// Return the direct effects (children) of a record.
    pub fn effects_of(&self, id: &RecordId) -> Vec<RecordId> {
        self.forward.get(id).map(|v| v.clone()).unwrap_or_default()
    }

    /// Return the direct causes (parents) of a record.
    pub fn causes_of(&self, id: &RecordId) -> Vec<RecordId> {
        self.reverse.get(id).map(|v| v.clone()).unwrap_or_default()
    }

    /// Return all records that have no causes (roots of the DAG).
    pub fn roots(&self) -> Vec<RecordId> {
        self.reverse
            .iter()
            .filter(|entry| entry.value().is_empty())
            .map(|entry| *entry.key())
            .collect()
    }

    /// Return all records that have no effects (leaves of the DAG).
    pub fn leaves(&self) -> Vec<RecordId> {
        self.forward
            .iter()
            .filter(|entry| entry.value().is_empty())
            .map(|entry| *entry.key())
            .collect()
    }

    /// Compute the shortest causal path between two records.
    /// Returns None if no path exists.
    pub fn shortest_path(&self, from: &RecordId, to: &RecordId) -> Option<Vec<RecordId>> {
        let mut queue: VecDeque<Vec<RecordId>> = VecDeque::new();
        let mut visited: HashSet<RecordId> = HashSet::new();

        queue.push_back(vec![*from]);
        visited.insert(*from);

        while let Some(path) = queue.pop_front() {
            let current = *path.last().unwrap();
            if &current == to {
                return Some(path);
            }
            let neighbors = self.forward.get(&current).map(|v| v.clone()).unwrap_or_default();
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push_back(new_path);
                }
            }
        }
        None
    }

    /// Total number of causal edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edge_count.load(Ordering::Relaxed)
    }

    /// Total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.forward.len()
    }

    fn bfs(&self, start: &RecordId, max_depth: usize, forward: bool) -> Vec<RecordId> {
        let mut result = Vec::new();
        let mut visited: HashSet<RecordId> = HashSet::new();
        let mut queue: VecDeque<(RecordId, usize)> = VecDeque::new();

        queue.push_back((*start, 0));
        visited.insert(*start);

        while let Some((node, depth)) = queue.pop_front() {
            if depth > 0 {
                result.push(node);
            }
            if depth >= max_depth {
                continue;
            }
            let neighbors = if forward {
                self.forward.get(&node).map(|v| v.clone()).unwrap_or_default()
            } else {
                self.reverse.get(&node).map(|v| v.clone()).unwrap_or_default()
            };
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
        result
    }
}

impl Default for CausalGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(n: u8) -> RecordId {
        let mut x = [0u8; 32];
        x[0] = n;
        x
    }

    #[test]
    fn descendants_traverses_chain() {
        let g = CausalGraph::new();
        // A → B → C → D
        g.add_node(id(0));
        g.add_edge(id(0), id(1));
        g.add_edge(id(1), id(2));
        g.add_edge(id(2), id(3));

        let d = g.descendants(&id(0), 10);
        assert_eq!(d.len(), 3);
        assert!(d.contains(&id(1)));
        assert!(d.contains(&id(2)));
        assert!(d.contains(&id(3)));
    }

    #[test]
    fn descendants_respects_depth() {
        let g = CausalGraph::new();
        g.add_node(id(0));
        g.add_edge(id(0), id(1));
        g.add_edge(id(1), id(2));
        g.add_edge(id(2), id(3));

        let d = g.descendants(&id(0), 2);
        assert_eq!(d.len(), 2);
        assert!(d.contains(&id(1)));
        assert!(d.contains(&id(2)));
        assert!(!d.contains(&id(3)));
    }

    #[test]
    fn ancestors_traverses_backward() {
        let g = CausalGraph::new();
        g.add_edge(id(0), id(1));
        g.add_edge(id(1), id(2));

        let ancs = g.ancestors(&id(2), 10);
        assert!(ancs.contains(&id(1)));
        assert!(ancs.contains(&id(0)));
    }

    #[test]
    fn shortest_path_finds_path() {
        let g = CausalGraph::new();
        g.add_edge(id(0), id(1));
        g.add_edge(id(1), id(2));
        g.add_edge(id(0), id(2)); // shortcut

        let path = g.shortest_path(&id(0), &id(2)).unwrap();
        assert_eq!(path.len(), 2); // [0, 2] is shorter than [0, 1, 2]
    }

    #[test]
    fn shortest_path_returns_none_for_disconnected() {
        let g = CausalGraph::new();
        g.add_node(id(0));
        g.add_node(id(5));

        assert!(g.shortest_path(&id(0), &id(5)).is_none());
    }

    #[test]
    fn roots_and_leaves() {
        let g = CausalGraph::new();
        g.add_node(id(0));
        g.add_node(id(3));
        g.add_edge(id(0), id(1));
        g.add_edge(id(1), id(2));
        g.add_edge(id(2), id(3));

        let roots = g.roots();
        assert!(roots.contains(&id(0)));

        let leaves = g.leaves();
        assert!(leaves.contains(&id(3)));
    }
}
