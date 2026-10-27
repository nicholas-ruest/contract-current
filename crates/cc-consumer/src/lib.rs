use cc_errors::{CurrentError, Result};
use cc_types::Consumer;
pub fn synthetic_registry() -> Vec<Consumer> {
    vec![
        Consumer {
            id: "consumer-checkout".into(),
            name: "Checkout Web".into(),
            roles: vec!["analyzer".into()],
            score: 0.92,
        },
        Consumer {
            id: "consumer-billing".into(),
            name: "Billing Worker".into(),
            roles: vec!["migrator".into()],
            score: 0.88,
        },
        Consumer {
            id: "consumer-ledger".into(),
            name: "Ledger Export".into(),
            roles: vec!["verifier".into()],
            score: 0.86,
        },
    ]
}
pub fn by_role<'a>(consumers: &'a [Consumer], role: &str) -> Result<&'a Consumer> {
    consumers
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == role))
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .ok_or_else(|| CurrentError::Dependency(format!("no consumer for {role}")))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_role() {
        assert_eq!(
            by_role(&synthetic_registry(), "migrator").unwrap().name,
            "Billing Worker"
        )
    }
}
