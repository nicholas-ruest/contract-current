use cc_consumer::by_role;
use cc_errors::Result;
use cc_types::{Consumer, ImpactNode};
pub fn assemble(impacts: &[ImpactNode], registry: &[Consumer]) -> Result<Vec<Consumer>> {
    let mut out = Vec::new();
    for t in impacts {
        let a = by_role(registry, &t.role)?.clone();
        if !out.iter().any(|x: &Consumer| x.id == a.id) {
            out.push(a)
        }
    }
    Ok(out)
}
pub fn separated(plan: &[Consumer]) -> bool {
    let reviewers: Vec<_> = plan
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "migrator"))
        .collect();
    let operators: Vec<_> = plan
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "verifier"))
        .collect();
    reviewers
        .iter()
        .all(|r| operators.iter().all(|o| r.id != o.id))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duties_separate() {
        assert!(separated(&cc_consumer::synthetic_registry()))
    }
}
