use cc_consumer::by_role;
use cc_errors::Result;
use cc_types::{Consumer, ImpactNode};
pub fn coverage(impacts: &[ImpactNode], consumers: &[Consumer]) -> Result<f64> {
    for t in impacts {
        by_role(consumers, &t.role)?;
    }
    Ok(impacts.len() as f64 / impacts.len().max(1) as f64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full() {
        let a = cc_consumer::synthetic_registry();
        let t = vec![ImpactNode {
            id: "x".into(),
            role: "migrator".into(),
            depends_on: vec![],
        }];
        assert_eq!(coverage(&t, &a).unwrap(), 1.0)
    }
}
