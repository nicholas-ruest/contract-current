use cc_consumer::synthetic_registry;
use cc_types::Consumer;
pub fn deterministic_consumers() -> Vec<Consumer> {
    synthetic_registry()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        assert_eq!(deterministic_consumers()[0].id, "consumer-checkout")
    }
}
