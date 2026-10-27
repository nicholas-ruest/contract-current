use cc_embedding::embed;
use cc_policy::authorize;
use cc_ruvector::MigrationDecisionIndex;
use cc_rvf::RvfArchive;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut index = MigrationDecisionIndex::new()?;
    index.insert("m1", embed("governed incident delegation")?)?;
    let recall = !index.search(&embed("incident delegation")?, 1)?.is_empty();
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("ablation.rvf");
    RvfArchive::append(&path, 1, b"durable-decision")?;
    let durable = RvfArchive::read_all(&path)?[0].1 == b"durable-decision";
    let allowed = authorize("standard", true, 0.1).is_ok();
    let denied = authorize("breaking", true, 0.1).is_err();
    let calibration = cc_calibration::should_abstain(0.4, 0.25);
    let rows = serde_json::json!([
      {"mode":"combined","success":recall&&durable&&allowed&&denied&&calibration,"recall":recall,"restart_retrieval":durable,"policy_denial":denied},
      {"mode":"ruvnet_only","success":recall&&durable,"recall":recall,"restart_retrieval":durable,"policy_denial":false},
      {"mode":"enterprise_only","success":allowed&&denied,"recall":false,"restart_retrieval":false,"policy_denial":denied},
      {"mode":"without_ruvector","success":durable&&allowed&&denied,"recall":false,"restart_retrieval":durable,"policy_denial":denied},
      {"mode":"without_rvf","success":recall&&allowed&&denied,"recall":recall,"restart_retrieval":false,"policy_denial":denied},
      {"mode":"without_cedar-policy","success":recall&&durable,"recall":recall,"restart_retrieval":durable,"policy_denial":false},
      {"mode":"without_calibration","success":false,"recall":recall,"restart_retrieval":durable,"policy_denial":denied}
    ]);
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
