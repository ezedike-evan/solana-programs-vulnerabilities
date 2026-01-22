use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,
}

pub fn vulnerable_transfer(amount: u64, balance: u64) -> Result<u64> {
    // VULNERABLE: INTEGER UNDERFLOW
    //
    // Context: A user tries to withdraw 'amount' from 'balance'.
    //
    // Problem: We use `wrapping_sub` to simulate an environment where overflow checks
    // are disabled or where the developer explicitly ignored safety.
    // If amount > balance (e.g. 10 - 20), this wraps to a huge number (u64::MAX - 9).
    //
    // In a real exploit, this "new balance" would be saved to the account, 
    // giving the attacker near-infinite funds.
    
    let new_balance = balance.wrapping_sub(amount); // <--- Underflow risk

    msg!("Vulnerable Calc: {} - {} = {}", balance, amount, new_balance);
    
    Ok(new_balance)
}
