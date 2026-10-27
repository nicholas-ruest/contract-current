use cc_errors::Result;
use cc_types::{Consumer, ContractRevision, ImpactNode};
use sha2::{Digest, Sha256};
pub fn digest(
    revision: &ContractRevision,
    impacts: &[ImpactNode],
    plan: &[Consumer],
) -> Result<String> {
    let bytes = serde_json::to_vec(&(revision, impacts, plan))
        .map_err(|e| cc_errors::CurrentError::Invalid(e.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
pub fn scopes(impacts: &[ImpactNode]) -> Vec<String> {
    impacts
        .iter()
        .map(|t| format!("decision:{}:execute", t.id))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        let o = ContractRevision {
            text: "x".into(),
            change_class: "additive".into(),
        };
        assert_eq!(digest(&o, &[], &[]).unwrap(), digest(&o, &[], &[]).unwrap())
    }
}
