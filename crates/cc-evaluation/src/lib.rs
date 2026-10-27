use cc_types::MigrationDecision;
#[derive(Debug, Clone)]
pub struct Metrics {
    pub coverage: f64,
    pub risk: f64,
    pub plan_size: usize,
}
pub fn measure(m: &MigrationDecision) -> Metrics {
    Metrics {
        coverage: m.impacts.len() as f64 / m.impacts.len().max(1) as f64,
        risk: m.risk,
        plan_size: m.consumers.len(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metric_shape() {
        let m = MigrationDecision::new(
            cc_types::ContractRevision {
                text: "x".into(),
                change_class: "additive".into(),
            },
            vec![],
            vec![],
            0.2,
            "h".into(),
            "c".into(),
        );
        assert_eq!(measure(&m).plan_size, 0)
    }
}
