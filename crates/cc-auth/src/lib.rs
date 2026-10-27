use cc_errors::{CurrentError, Result};
use sha2::{Digest, Sha256};
pub fn verify(provided: &str, expected: &str) -> Result<()> {
    let a = Sha256::digest(provided.as_bytes());
    let b = Sha256::digest(expected.as_bytes());
    if a == b {
        Ok(())
    } else {
        Err(CurrentError::Unauthorized)
    }
}
pub fn principal(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))[..12].to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn auth() {
        assert!(verify("x", "x").is_ok());
        assert!(verify("x", "y").is_err())
    }
}
