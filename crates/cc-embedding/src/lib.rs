use cc_errors::{CurrentError, Result};
use sha2::{Digest, Sha256};
pub const DIMENSION: usize = 32;
pub fn embed(text: &str) -> Result<Vec<f32>> {
    if text.is_empty() {
        return Err(CurrentError::Invalid("empty embedding".into()));
    }
    let h = Sha256::digest(text.as_bytes());
    Ok(h.iter().map(|b| f32::from(*b) / 255.0).collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dimension() {
        assert_eq!(embed("revision").unwrap().len(), DIMENSION)
    }
}
