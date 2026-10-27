use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Compatibility {
    Additive,
    Breaking,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaChange {
    pub pointer: String,
    pub compatibility: Compatibility,
    pub reason: String,
}
pub fn classify_revision(revision: &str) -> Vec<SchemaChange> {
    let lower = revision.to_ascii_lowercase();
    let breaking = ["remove", "required", "rename", "narrow"]
        .iter()
        .any(|needle| lower.contains(needle));
    vec![SchemaChange {
        pointer: "/paths/payment".into(),
        compatibility: if breaking {
            Compatibility::Breaking
        } else {
            Compatibility::Additive
        },
        reason: if breaking {
            "consumer contract may reject the proposed shape".into()
        } else {
            "existing consumers retain their accepted shape".into()
        },
    }]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_required_field_as_breaking() {
        assert_eq!(
            classify_revision("add required auth field")[0].compatibility,
            Compatibility::Breaking
        );
        assert_eq!(
            classify_revision("add optional metadata")[0].compatibility,
            Compatibility::Additive
        );
    }
}
