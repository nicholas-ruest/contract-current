use cc_errors::{CurrentError, Result};
use cc_types::Consumer;
pub fn utility(plan: &[Consumer], cost_weight: f64) -> Result<f64> {
    if plan.is_empty() {
        return Err(CurrentError::Invalid("empty plan".into()));
    }
    Ok(
        plan.iter().map(|a| a.score).sum::<f64>() / plan.len() as f64
            - cost_weight * plan.len() as f64,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn penalizes_size() {
        let a = cc_consumer::synthetic_registry();
        assert!(utility(&a, 0.01).unwrap() < 1.0)
    }
}
