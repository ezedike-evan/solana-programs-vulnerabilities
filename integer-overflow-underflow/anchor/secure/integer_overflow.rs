use anchor_lang::prelude::*;
use crate::vulnerable::integer_overflow::ErrorCode;

pub fn secure_transfer(amount: u64, balance: u64) -> Result<u64> {
    // SECURE: CHECKED ARITHMETIC
    //
    // Context: A user tries to withdraw 'amount' from 'balance'.
    //
    // Fix: We use `.checked_sub()`. 
    // If the operation would underflow (amount > balance), it returns None.
    // We then convert that None into a specific error using `.ok_or()`.
    
    let new_balance = balance.checked_sub(amount)
        .ok_or(ErrorCode::InsufficientFunds)?; // <--- Returns error on underflow

    msg!("Secure Calc: {} - {} = {}", balance, amount, new_balance);

    Ok(new_balance)
}
