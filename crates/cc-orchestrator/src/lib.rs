use cc_errors::{CurrentError, Result};
use cc_repository::Repository;
use cc_ruvector::MigrationDecisionIndex;
use cc_types::MigrationDecision;
use std::{path::PathBuf, sync::Mutex};
pub struct Orchestrator {
    repo: Repository,
    index: Mutex<MigrationDecisionIndex>,
}
impl Orchestrator {
    pub fn open(path: PathBuf) -> Result<Self> {
        let repo = Repository::open(path)?;
        let mut index = MigrationDecisionIndex::new()?;
        for m in repo.list()? {
            index.insert(&m.id, cc_embedding::embed(&m.revision.text)?)?
        }
        Ok(Self {
            repo,
            index: Mutex::new(index),
        })
    }
    pub fn create(
        &self,
        text: &str,
        change_class: &str,
        correlation_id: String,
    ) -> Result<MigrationDecision> {
        if let Some(existing) = self
            .repo
            .list()?
            .into_iter()
            .find(|m| m.correlation_id == correlation_id)
        {
            return Ok(existing);
        }
        let revision = cc_contract::validate(text, change_class)?;
        let impacts = cc_impact_graph::decompose(&revision.text)?;
        if !cc_impact_graph::acyclic(&impacts) {
            return Err(CurrentError::Invalid("impact cycle".into()));
        }
        let registry = cc_seed::deterministic_consumers();
        let _coverage = cc_compatibility::coverage(&impacts, &registry)?;
        let plan = cc_migration_plan::assemble(&impacts, &registry)?;
        let separated = cc_migration_plan::separated(&plan);
        let risk =
            cc_calibration::conformal_risk(&plan.iter().map(|a| a.score).collect::<Vec<_>>(), 0.1)?;
        if cc_calibration::should_abstain(risk, 0.25) {
            return Err(CurrentError::Denied("calibration abstained".into()));
        }
        cc_policy::authorize(&revision.change_class, separated, risk)?;
        let _utility = cc_wave_optimizer::utility(&plan, 0.01)?;
        let _budgets = cc_risk_budget::allocate(impacts.len(), 90)?;
        let contract_hash = cc_decision::digest(&revision, &impacts, &plan)?;
        let _scopes = cc_decision::scopes(&impacts);
        let mut decision =
            MigrationDecision::new(revision, impacts, plan, risk, contract_hash, correlation_id);
        let analogues = {
            let idx = self
                .index
                .lock()
                .map_err(|_| CurrentError::Dependency("index lock".into()))?;
            cc_recall::recall(&idx, &decision.revision.text)?
        };
        if !analogues.is_empty() {
            decision.status = "authorized-with-analogue".into()
        }
        self.repo.save(&decision)?;
        self.index
            .lock()
            .map_err(|_| CurrentError::Dependency("index lock".into()))?
            .insert(&decision.id, cc_embedding::embed(&decision.revision.text)?)?;
        Ok(decision)
    }
    pub fn list(&self) -> Result<Vec<MigrationDecision>> {
        self.repo.list()
    }
    pub fn get(&self, id: &str) -> Result<MigrationDecision> {
        self.repo.get(id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertical_slice() {
        let d = tempfile::tempdir().unwrap();
        let o = Orchestrator::open(d.path().join("x.rvf")).unwrap();
        let m = o
            .create(
                "Add optional delivery_window field to checkout response",
                "standard",
                "test-corr".into(),
            )
            .unwrap();
        assert_eq!(o.get(&m.id).unwrap().consumers.len(), 3);
        assert!(o
            .create(
                "Remove required customer_id from checkout response",
                "breaking",
                "c".into()
            )
            .is_err())
    }
}
