use anchor_lang::prelude::*;

pub fn secure_calculate_reward(amount: u64, rate_bps: u64) -> Result<u64> {
    // SECURE: ORDER OF OPERATIONS FIXED
    //
    // Logic: Calculate reward = (amount * rate_bps) / 10,000
    //
    // FIX: We multiply the amount by the rate FIRST.
    // This scales the value up, preserving the data that would be lost
    // as a remainder if we divided first.
    //
    // Best Practice: We cast to u128 before multiplying to eliminate the risk
    // of integer overflow (which would cause the transaction to fail/revert)
    // if `amount * rate_bps` exceeds the u64 limit.
    
    let numerator = (amount as u128).checked_mul(rate_bps as u128).unwrap();
    let reward = numerator.checked_div(10000).unwrap();

    msg!("Secure Calc: ({} * {}) / 10000 = {}", amount, rate_bps, reward as u64);

    Ok(reward as u64)
}
