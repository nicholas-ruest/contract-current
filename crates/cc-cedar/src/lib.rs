use cc_errors::{CurrentError, Result};
use cedar_policy::{Authorizer, Context, Decision, Entities, PolicySet, Request};
use std::str::FromStr;

pub fn allow(policy: &str, change_class: &str, separated: bool, risk: f64) -> Result<bool> {
    let policies = PolicySet::from_str(policy)
        .map_err(|e| CurrentError::Dependency(format!("Cedar policy: {e}")))?;
    let principal = if separated && risk <= 0.25 {
        "release-manager"
    } else {
        "blocked"
    };
    let action = if change_class == "breaking" {
        "breaking-change"
    } else {
        "migrate"
    };
    let p = format!("User::\"{principal}\"")
        .parse()
        .map_err(|e: cedar_policy::ParseErrors| CurrentError::Invalid(e.to_string()))?;
    let a = format!("Action::\"{action}\"")
        .parse()
        .map_err(|e: cedar_policy::ParseErrors| CurrentError::Invalid(e.to_string()))?;
    let r = "Api::\"payment\""
        .parse()
        .map_err(|e: cedar_policy::ParseErrors| CurrentError::Invalid(e.to_string()))?;
    let request = Request::new(p, a, r, Context::empty(), None)
        .map_err(|e| CurrentError::Invalid(e.to_string()))?;
    let response = Authorizer::new().is_authorized(&request, &policies, &Entities::empty());
    Ok(response.decision() == Decision::Allow)
}

#[cfg(test)]
mod tests {
    use super::*;
    const POLICY: &str = include_str!("../../../policies/change.cedar");
    #[test]
    fn cedar_permits_safe_change_and_denies_breaking_change() {
        assert!(allow(POLICY, "standard", true, 0.1).unwrap());
        assert!(!allow(POLICY, "breaking", true, 0.1).unwrap());
        assert!(!allow(POLICY, "standard", false, 0.1).unwrap());
    }
}
