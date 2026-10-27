use cc_errors::{CurrentError, Result};
use cc_rvf::RvfArchive;
use cc_types::MigrationDecision;
use sha2::{Digest, Sha256};
use std::path::Path;
#[derive(serde::Serialize, serde::Deserialize)]
struct Envelope {
    decision: MigrationDecision,
    previous: String,
    hash: String,
}
pub fn append(
    path: &Path,
    seq: u64,
    decision: &MigrationDecision,
    previous: &str,
) -> Result<String> {
    let body = serde_json::to_vec(&(decision, previous))
        .map_err(|e| CurrentError::Persistence(e.to_string()))?;
    let hash = hex::encode(Sha256::digest(body));
    let e = Envelope {
        decision: decision.clone(),
        previous: previous.into(),
        hash: hash.clone(),
    };
    RvfArchive::append(
        path,
        seq,
        &serde_json::to_vec(&e).map_err(|x| CurrentError::Persistence(x.to_string()))?,
    )?;
    Ok(hash)
}
pub fn load(path: &Path) -> Result<Vec<MigrationDecision>> {
    RvfArchive::read_all(path)?
        .into_iter()
        .map(|(_, b)| {
            serde_json::from_slice::<Envelope>(&b)
                .map(|e| e.decision)
                .map_err(|e| CurrentError::Persistence(e.to_string()))
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let d = tempfile::tempdir().unwrap();
        assert!(load(&d.path().join("x")).unwrap().is_empty())
    }
}
