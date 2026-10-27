use cc_audit::{append, load};
use cc_errors::{CurrentError, Result};
use cc_types::MigrationDecision;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
pub struct Repository {
    path: PathBuf,
    seq: AtomicU64,
}
impl Repository {
    pub fn open(path: PathBuf) -> Result<Self> {
        let n = load(&path)?.len() as u64;
        Ok(Self {
            path,
            seq: AtomicU64::new(n),
        })
    }
    pub fn save(&self, m: &MigrationDecision) -> Result<()> {
        let seq = self.seq.fetch_add(1, Ordering::SeqCst) + 1;
        let previous = load(&self.path)?
            .last()
            .map(|x| x.contract_hash.clone())
            .unwrap_or_default();
        append(&self.path, seq, m, &previous)?;
        Ok(())
    }
    pub fn list(&self) -> Result<Vec<MigrationDecision>> {
        load(&self.path)
    }
    pub fn get(&self, id: &str) -> Result<MigrationDecision> {
        self.list()?
            .into_iter()
            .find(|m| m.id == id)
            .ok_or_else(|| CurrentError::NotFound(id.into()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing() {
        let d = tempfile::tempdir().unwrap();
        let r = Repository::open(d.path().join("x.rvf")).unwrap();
        assert!(r.get("nope").is_err())
    }
}
