use cc_auth::verify;
use cc_errors::Result;
use uuid::Uuid;
pub fn authorize(header: Option<&str>, expected: &str) -> Result<()> {
    let token = header
        .and_then(|h| h.strip_prefix("Bearer "))
        .unwrap_or_default();
    verify(token, expected)
}
pub fn correlation(header: Option<&str>) -> String {
    header
        .filter(|x| !x.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correlation_created() {
        assert!(!correlation(None).is_empty())
    }
}
