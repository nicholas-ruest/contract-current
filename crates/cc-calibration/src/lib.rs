use cc_errors::{CurrentError, Result};
pub fn conformal_risk(scores: &[f64], alpha: f64) -> Result<f64> {
    if scores.is_empty() || !(0.0..1.0).contains(&alpha) {
        return Err(CurrentError::Invalid("calibration inputs".into()));
    }
    let mut v = scores.to_vec();
    v.sort_by(f64::total_cmp);
    let i = ((v.len() as f64 * (1.0 - alpha)).ceil() as usize)
        .saturating_sub(1)
        .min(v.len() - 1);
    Ok(1.0 - v[i])
}
pub fn should_abstain(risk: f64, limit: f64) -> bool {
    risk > limit
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abstains() {
        assert!(should_abstain(0.3, 0.2))
    }
}
