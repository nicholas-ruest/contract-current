use cc_errors::{CurrentError, Result};
pub const POLICY: &str = include_str!("../../../policies/change.cedar");
pub fn authorize(change_class: &str, separated: bool, risk: f64) -> Result<()> {
    if cc_cedar::allow(POLICY, change_class, separated, risk)? {
        Ok(())
    } else {
        Err(CurrentError::Denied(
            "decision policy rejected revision".into(),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restricted_denied() {
        assert!(authorize("breaking", true, 0.1).is_err())
    }
}
