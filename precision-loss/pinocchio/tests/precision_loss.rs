#[cfg(test)]
mod tests {
    // We include the module files directly for testing purposes.
    // In a real crate, these would be in lib.rs
    #[path = "../vulnerable/integer_division.rs"]
    mod vulnerable;

    #[path = "../secure/integer_division.rs"]
    mod secure;

    // Constants for the test case
    const AMOUNT_SMALL: u64 = 9999; // < 10,000 divisor
    const RATE_BPS: u64 = 5000;     // 50%
    const EXPECTED_REWARD: u64 = 4999; // floor(9999 * 0.5)

    #[test]
    fn test_vulnerable_precision_loss() {
        // Business Requirement: Calculate 50% of 9999.
        // Expected: 4999.
        
        let result = vulnerable::calculate_reward(AMOUNT_SMALL, RATE_BPS).unwrap();
        
        println!("Vulnerable Result: {}", result);
        
        // This assertion FAILS because the result is 0.
        assert_eq!(result, EXPECTED_REWARD, "Vulnerable implementation lost precision!");
    }

    #[test]
    fn test_secure_precision_correct() {
        // Business Requirement: Calculate 50% of 9999.
        // Expected: 4999.
        
        let result = secure::calculate_reward(AMOUNT_SMALL, RATE_BPS).unwrap();
        
        println!("Secure Result: {}", result);
        
        // This assertion PASSES.
        assert_eq!(result, EXPECTED_REWARD);
    }
}
