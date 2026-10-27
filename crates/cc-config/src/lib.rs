use cc_errors::{CurrentError, Result};
use std::path::PathBuf;
#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,
    pub data_path: PathBuf,
    pub token: String,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let bind = std::env::var("CC_BIND").unwrap_or_else(|_| "127.0.0.1:18080".into());
        if !bind.contains(':') {
            return Err(CurrentError::Invalid("CC_BIND".into()));
        }
        Ok(Self {
            bind,
            data_path: std::env::var("CC_DATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| "data/decisions.rvf".into()),
            token: std::env::var("CC_TOKEN").unwrap_or_else(|_| "demo-token".into()),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults() {
        assert!(Config::from_env().unwrap().bind.ends_with("8080"))
    }
}
