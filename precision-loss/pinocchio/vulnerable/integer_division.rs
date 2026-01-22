use solana_program::{msg, program_error::ProgramError};

/// Vulnerable implementation of reward calculation using native Rust.
///
/// Logic: amount * rate / 10000
pub fn calculate_reward(
    amount: u64,
    rate_bps: u64,
) -> Result<u64, ProgramError> {
    msg!("Computing reward (Vulnerable)...");

    // VULNERABLE: PRECISION LOSS
    // Dividing before multiplying discards the remainder.
    // If amount < 10,000, this results in 0.
    
    let step1 = amount
        .checked_div(10000)
        .ok_or(ProgramError::ArithmeticOverflow)?; // Truncation happens here

    let reward = step1
        .checked_mul(rate_bps)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!("Result: {}", reward);

    Ok(reward)
}
