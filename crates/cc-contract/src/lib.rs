use cc_errors::{CurrentError, Result};
use cc_types::ContractRevision;
pub fn validate(text: &str, change_class: &str) -> Result<ContractRevision> {
    let t = text.trim();
    if t.len() < 12 {
        return Err(CurrentError::Invalid(
            "revision must be at least 12 characters".into(),
        ));
    }
    if !["additive", "standard", "breaking"].contains(&change_class) {
        return Err(CurrentError::Invalid("unknown change_class".into()));
    }
    Ok(ContractRevision {
        text: t.into(),
        change_class: change_class.into(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_tiny() {
        assert!(validate("tiny", "additive").is_err())
    }
}
