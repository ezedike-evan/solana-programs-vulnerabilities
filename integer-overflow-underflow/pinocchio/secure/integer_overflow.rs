use solana_program::{msg, program_error::ProgramError};

/// Secure implementation using checked math.
pub fn transfer(
    amount: u64,
    balance: u64,
) -> Result<u64, ProgramError> {
    msg!("Performing transfer (Secure)...");

    // SECURE: CHECKED SUBTRACTION
    // If amount > balance, checked_sub returns None.
    // We map this to an InsufficientFunds error (simulated here as CustomError or generic).
    
    let new_balance = balance
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;

    msg!("Result: {}", new_balance);

    Ok(new_balance)
}
