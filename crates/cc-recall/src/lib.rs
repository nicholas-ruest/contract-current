use cc_embedding::embed;
use cc_errors::Result;
use cc_ruvector::MigrationDecisionIndex;
use cc_types::Analogue;
pub fn recall(index: &MigrationDecisionIndex, text: &str) -> Result<Vec<Analogue>> {
    index.search(&embed(text)?, 3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_index() {
        assert!(recall(&MigrationDecisionIndex::new().unwrap(), "revision")
            .unwrap()
            .is_empty())
    }
}
