use anchor_lang::prelude::*;

pub fn vulnerable_calculate_reward(amount: u64, rate_bps: u64) -> Result<u64> {
    // VULNERABLE: PRECISION LOSS
    // 
    // Logic: Calculate reward = amount * (rate_bps / 10,000)
    //
    // PROBLEM: Dividing `amount` by 10,000 FIRST performs integer truncation.
    // If `amount` is < 10,000 (e.g., 5000 lamports), the result is 0.
    // 0 * rate_bps = 0.
    //
    // Even for larger numbers, the remainder is discarded, causing value leaks.
    // e.g. 19,000 / 10,000 = 1. The 9,000 remainder is lost.
    
    let intermediate = amount.checked_div(10000).unwrap(); // <--- Truncation here
    let reward = intermediate.checked_mul(rate_bps).unwrap();

    msg!("Vulnerable Calc: {} / 10000 * {} = {}", amount, rate_bps, reward);
    
    Ok(reward)
}
