use solana_program::{msg, program_error::ProgramError};

/// Vulnerable implementation using explicit wrapping to simulate unsafe behavior.
pub fn transfer(
    amount: u64,
    balance: u64,
) -> Result<u64, ProgramError> {
    msg!("Performing transfer (Vulnerable)...");

    // VULNERABLE: WRAPPING SUBTRACTION
    // We simulate an unchecked environment.
    // If amount > balance, this wraps around.
    
    let new_balance = balance.wrapping_sub(amount);

    msg!("Result: {}", new_balance);

    Ok(new_balance)
}
