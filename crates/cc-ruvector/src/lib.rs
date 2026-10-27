use cc_embedding::DIMENSION;
use cc_errors::{CurrentError, Result};
use cc_types::Analogue;
use ruvector_core::{
    types::{DbOptions, DistanceMetric, SearchQuery, VectorEntry},
    VectorDB,
};
use std::collections::HashMap;
pub struct MigrationDecisionIndex {
    db: VectorDB,
    len: usize,
}
impl MigrationDecisionIndex {
    pub fn new() -> Result<Self> {
        let db = VectorDB::new(DbOptions {
            dimensions: DIMENSION,
            distance_metric: DistanceMetric::Euclidean,
            storage_path: "memory://contract-current".into(),
            hnsw_config: None,
            quantization: None,
        })
        .map_err(|e| CurrentError::Dependency(format!("RuVector create: {e}")))?;
        Ok(Self { db, len: 0 })
    }
    pub fn insert(&mut self, id: &str, vector: Vec<f32>) -> Result<()> {
        let mut metadata = HashMap::new();
        metadata.insert("kind".into(), serde_json::json!("decision"));
        self.db
            .insert(VectorEntry {
                id: Some(id.into()),
                vector,
                metadata: Some(metadata),
            })
            .map_err(|e| CurrentError::Dependency(format!("RuVector insert: {e}")))?;
        self.len += 1;
        Ok(())
    }
    pub fn search(&self, vector: &[f32], k: usize) -> Result<Vec<Analogue>> {
        if self.len == 0 {
            return Ok(vec![]);
        }
        let hits = self
            .db
            .search(SearchQuery {
                vector: vector.to_vec(),
                k: k.min(self.len),
                filter: None,
                ef_search: None,
            })
            .map_err(|e| CurrentError::Dependency(format!("RuVector search: {e}")))?;
        Ok(hits
            .into_iter()
            .map(|h| Analogue {
                decision_id: h.id,
                distance: f64::from(h.score),
            })
            .collect())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn executed_search() {
        let mut i = MigrationDecisionIndex::new().unwrap();
        i.insert("m1", vec![0.0; DIMENSION]).unwrap();
        assert_eq!(i.search(&[0.1; DIMENSION], 1).unwrap()[0].decision_id, "m1")
    }
}
