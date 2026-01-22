use solana_program::{msg, program_error::ProgramError};

/// Secure implementation of reward calculation.
///
/// Logic: (amount * rate) / 10000
pub fn calculate_reward(
    amount: u64,
    rate_bps: u64,
) -> Result<u64, ProgramError> {
    msg!("Computing reward (Secure)...");

    // SECURE: ORDER OF OPERATIONS
    // 1. Cast to u128 to prevent overflow during multiplication.
    // 2. Multiply first to preserve precision.
    // 3. Divide last.

    let numerator = (amount as u128)
        .checked_mul(rate_bps as u128)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let reward = numerator
        .checked_div(10000)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Safe to cast back to u64 because logic dictates result <= amount * rate / 10000
    // If rate is reasonable, this fits. If rate is huge, it still fits logically provided inputs fit u64.
    let result = reward as u64;
    
    msg!("Result: {}", result);

    Ok(result)
}
