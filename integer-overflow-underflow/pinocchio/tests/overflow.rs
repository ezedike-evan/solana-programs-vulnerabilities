use solana_program::program_error::ProgramError;

#[cfg(test)]
mod tests {
    #[path = "../vulnerable/integer_overflow.rs"]
    mod vulnerable;

    #[path = "../secure/integer_overflow.rs"]
    mod secure;

    const INITIAL_BALANCE: u64 = 100;
    const WITHDRAW_AMOUNT_OVERSHOOT: u64 = 200;

    #[test]
    fn test_vulnerable_overflow() {
        // Attack: Withdraw 200 from 100.
        // Expected: Wraps to huge value.
        
        let result = vulnerable::transfer(WITHDRAW_AMOUNT_OVERSHOOT, INITIAL_BALANCE).unwrap();
        
        println!("Vulnerable New Balance: {}", result);
        
        // Assert that we have magically created money (result > initial)
        assert!(result > INITIAL_BALANCE, "Vulnerable implementation did not wrap!");
    }

    #[test]
    fn test_secure_overflow_rejection() {
        // Attack: Withdraw 200 from 100.
        // Expected: Error (InsufficientFunds).
        
        let result = secure::transfer(WITHDRAW_AMOUNT_OVERSHOOT, INITIAL_BALANCE);
        
        match result {
            Ok(val) => panic!("Secure implementation allowed underflow! Value: {}", val),
            Err(e) => {
                println!("Secure implementation correctly returned error: {:?}", e);
                assert_eq!(e, ProgramError::InsufficientFunds);
            }
        }
    }
}
