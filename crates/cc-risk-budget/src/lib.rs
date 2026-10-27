use cc_errors::{CurrentError, Result};
pub fn allocate(impacts: usize, total: u32) -> Result<Vec<u32>> {
    if impacts == 0 || total < impacts as u32 {
        return Err(CurrentError::Invalid("insufficient budget".into()));
    }
    let each = total / impacts as u32;
    Ok((0..impacts).map(|_| each).collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded() {
        assert_eq!(allocate(3, 90).unwrap(), vec![30, 30, 30])
    }
}
