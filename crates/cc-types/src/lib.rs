use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContractRevision {
    pub text: String,
    pub change_class: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImpactNode {
    pub id: String,
    pub role: String,
    pub depends_on: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Consumer {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
    pub score: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MigrationDecision {
    pub id: String,
    pub revision: ContractRevision,
    pub impacts: Vec<ImpactNode>,
    pub consumers: Vec<Consumer>,
    pub risk: f64,
    pub contract_hash: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub correlation_id: String,
}
impl MigrationDecision {
    pub fn new(
        revision: ContractRevision,
        impacts: Vec<ImpactNode>,
        consumers: Vec<Consumer>,
        risk: f64,
        contract_hash: String,
        correlation_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            revision,
            impacts,
            consumers,
            risk,
            contract_hash,
            created_at: Utc::now(),
            status: "authorized".into(),
            correlation_id,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMigrationDecision {
    pub revision: String,
    pub change_class: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub persistence: String,
    pub policy: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analogue {
    pub decision_id: String,
    pub distance: f64,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decision_has_identity() {
        let m = MigrationDecision::new(
            ContractRevision {
                text: "x".into(),
                change_class: "standard".into(),
            },
            vec![],
            vec![],
            0.1,
            "h".into(),
            "c".into(),
        );
        assert!(!m.id.is_empty())
    }
}
