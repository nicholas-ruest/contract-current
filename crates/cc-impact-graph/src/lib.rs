use cc_errors::{CurrentError, Result};
use cc_schema_diff::{classify_revision, Compatibility};
use cc_types::ImpactNode;
pub fn decompose(text: &str) -> Result<Vec<ImpactNode>> {
    if text.trim().is_empty() {
        return Err(CurrentError::Invalid("empty revision".into()));
    }
    let analysis = if matches!(
        classify_revision(text)[0].compatibility,
        Compatibility::Breaking
    ) {
        "breaking-analysis".to_string()
    } else {
        "compatible-analysis".to_string()
    };
    Ok(vec![
        ImpactNode {
            id: analysis.clone(),
            role: "analyzer".into(),
            depends_on: vec![],
        },
        ImpactNode {
            id: "verify".into(),
            role: "migrator".into(),
            depends_on: vec![analysis],
        },
        ImpactNode {
            id: "deliver".into(),
            role: "verifier".into(),
            depends_on: vec!["verify".into()],
        },
    ])
}
pub fn acyclic(impacts: &[ImpactNode]) -> bool {
    impacts.iter().enumerate().all(|(i, t)| {
        t.depends_on
            .iter()
            .all(|d| impacts[..i].iter().any(|x| &x.id == d))
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph_is_ordered() {
        let g = decompose("long enough revision").unwrap();
        assert!(acyclic(&g))
    }
}
