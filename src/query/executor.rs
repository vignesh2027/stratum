use crate::error::Result;
use crate::record::Record;
use crate::Stratum;
use super::parser::{AqslQuery, Clause, OrderDir};

/// Execute a parsed AQSL query against an Akasha database instance.
pub async fn execute(query: AqslQuery, db: &Stratum) -> Result<Vec<Record>> {
    // Step 1: Collect candidate sets from each clause.
    // For multi-clause queries, we intersect the results.
    let mut candidate_sets: Vec<Vec<Record>> = Vec::new();

    for clause in &query.clauses {
        let records = execute_clause(clause, db).await?;
        candidate_sets.push(records);
    }

    // Step 2: Intersect all candidate sets by record ID.
    let records = if candidate_sets.is_empty() {
        // No WHERE clauses: full scan
        db.storage.scan_all()?.collect()
    } else if candidate_sets.len() == 1 {
        candidate_sets.remove(0)
    } else {
        intersect_record_sets(candidate_sets)
    };

    // Step 3: Sort.
    let mut records = records;
    match query.order {
        OrderDir::TimeAsc => records.sort_by_key(|r| r.timestamp),
        OrderDir::TimeDesc => records.sort_by_key(|r| std::cmp::Reverse(r.timestamp)),
        OrderDir::RelevanceDesc => {
            // Relevance order is preserved from semantic search; keep as-is.
        }
    }

    // Step 4: Apply offset and limit.
    let records = records
        .into_iter()
        .skip(query.offset)
        .take(query.limit)
        .collect();

    Ok(records)
}

async fn execute_clause(clause: &Clause, db: &Stratum) -> Result<Vec<Record>> {
    match clause {
        Clause::TimeRange { from, to } => {
            let from_ns = from.to_nanos();
            let to_ns = to.to_nanos();
            let ids = db.temporal.range(from_ns, to_ns, usize::MAX);
            let mut records = Vec::new();
            for id in ids {
                if let Some(r) = db.storage.get(&id)? {
                    records.push(r);
                }
            }
            Ok(records)
        }
        Clause::TimeAfter(t) => {
            let ts = t.to_nanos();
            let ids = db.temporal.after(ts, usize::MAX);
            fetch_by_ids(db, &ids)
        }
        Clause::TimeBefore(t) => {
            let ts = t.to_nanos();
            let ids = db.temporal.before(ts, usize::MAX);
            fetch_by_ids(db, &ids)
        }
        Clause::SimilarTo { embedding, threshold } => {
            let hits = db.semantic.search(embedding, usize::MAX, *threshold);
            let mut records = Vec::new();
            for (id, _score) in hits {
                if let Some(r) = db.storage.get(&id)? {
                    records.push(r);
                }
            }
            Ok(records)
        }
        Clause::CausedBy { id, depth } => {
            let ids = db.causal.descendants(id, *depth);
            fetch_by_ids(db, &ids)
        }
        Clause::Causes { id, depth } => {
            let ids = db.causal.ancestors(id, *depth);
            fetch_by_ids(db, &ids)
        }
        Clause::Schema(schema) => {
            let all: Vec<Record> = db.storage.scan_all()?.collect();
            Ok(all.into_iter().filter(|r| &r.schema == schema).collect())
        }
        Clause::Tag { key, value } => {
            let all: Vec<Record> = db.storage.scan_all()?.collect();
            Ok(all.into_iter()
                .filter(|r| r.metadata.get(key).map(|v| v == value).unwrap_or(false))
                .collect())
        }
    }
}

fn fetch_by_ids(db: &Stratum, ids: &[crate::record::RecordId]) -> Result<Vec<Record>> {
    let mut records = Vec::new();
    for id in ids {
        if let Some(r) = db.storage.get(id)? {
            records.push(r);
        }
    }
    Ok(records)
}

fn intersect_record_sets(sets: Vec<Vec<Record>>) -> Vec<Record> {
    use std::collections::HashSet;
    if sets.is_empty() { return Vec::new(); }

    let id_sets: Vec<HashSet<[u8; 32]>> = sets.iter()
        .map(|s| s.iter().map(|r| r.id).collect())
        .collect();

    let common_ids: HashSet<[u8; 32]> = id_sets[0].iter()
        .filter(|id| id_sets[1..].iter().all(|s| s.contains(*id)))
        .copied()
        .collect();

    sets.into_iter()
        .next()
        .unwrap_or_default()
        .into_iter()
        .filter(|r| common_ids.contains(&r.id))
        .collect()
}


