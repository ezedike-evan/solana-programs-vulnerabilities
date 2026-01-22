# Integer Overflow / Underflow

## 1. High-Level Explanation
Integer overflow and underflow occur when an arithmetic operation results in a numeric value that is outside the range of the data type used to store it.
- **Overflow:** A value exceeds the maximum limit (e.g., `u8` max is 255. `255 + 1 = 0` or panics).
- **Underflow:** A value drops below the minimum limit (e.g., `u8` min is 0. `0 - 1 = 255` or panics).

In Solana, where `u64` is the standard for token balances (lamports), unchecked arithmetic can lead to catastrophic bugs where users withdraw more than they own (underflow wrapping to huge balance) or where total supply calculations wrap around to zero (overflow).

## 2. Why This Issue is Dangerous in Solana
While modern Rust (with `overflow-checks = true`) panics on overflow, many optimized or older Solana programs disable these checks to save compute units (CU). Furthermore, developers might explicitly use `wrapping_` arithmetic logic thinking it's safe, or migrate code from other languages (like Solidity <0.8.0) without understanding Rust's behavior in Release profiles.
- **Infinite Money Glitch:** Underflowing a balance calculation (`balance - amount`) can grant a user `u64::MAX` tokens.
- **Locked Funds:** Overflowing a supply counter could reset the total supply to a small number, breaking invariant checks and freezing protocols.

## 3. Root Cause Analysis
The root cause is the fixed-width nature of computer integers combined with assumptions about arithmetic safety.
- **Release vs. Debug:** By default, Rust checks for overflow in Debug builds but *may* wrap in Release builds if `overflow-checks` is not explicitly set to `true` in `Cargo.toml`.
- **Explicit Wrapping:** Using `wrapping_add` or `wrapping_sub` bypasses safety checks.
- **Unchecked Blocks:** Using `unsafe` blocks with unchecked arithmetic intrinsics.

In the context of this module, we demonstrate the risk when arithmetic is assumed safe but is not checked, leading to wrapping behavior.

## 4. Annotated Vulnerable Code Snippet
```rust
pub fn withdraw_vulnerable(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let account = &mut ctx.accounts.user_account;
    
    // VULNERABLE: If `amount` > `account.balance`, and validation is missing,
    // this could wrap (underflow) in an unchecked environment, 
    // resulting in a massive balance instead of an error.
    account.balance = account.balance.wrapping_sub(amount);
    
    Ok(())
}
```

## 5. Annotated Secure Code Snippet
```rust
pub fn withdraw_secure(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let account = &mut ctx.accounts.user_account;

    // SECURE: Use `checked_sub` which returns `None` on underflow.
    // We explicitly handle the error case.
    account.balance = account.balance.checked_sub(amount)
        .ok_or(error!(MyError::InsufficientFunds))?;
        
    Ok(())
}
```

## 6. Anchor vs Pinocchio Comparison

| Feature | Anchor | Pinocchio (Native) |
| :--- | :--- | :--- |
| **Arithmetic Defaults** | Defaults to `overflow-checks = true` in new workspaces, but can be disabled in `Cargo.toml`. | Defaults to `overflow-checks = true` usually, but often disabled for CU optimization. |
| **Safety Tools** | `checked_` math (standard Rust). | `checked_` math (standard Rust). |
| **Error Propagation** | `Result` based (e.g. `ok_or(ErrorCode)`). | `ProgramError` based. |
| **Risk** | High if user assumes framework handles math magic. | High if user manually optimizes math poorly. |

## 7. Security Patterns & Best Practices
1. **Always Use Checked Math:** Use `.checked_add()`, `.checked_sub()`, `.checked_mul()`, `.checked_div()` for all economic math.
2. **Avoid `wrapping_` methods:** Unless you strictly intend to implement modular arithmetic (e.g. cryptography), never use wrapping methods on balances.
3. **Verify `Cargo.toml`:** Ensure `[profile.release] overflow-checks = true` is set.
4. **Zero Checks:** Ensure logic handles 0 correctly.
5. **Checked Casts:** Use `u128` for intermediate calculations if `u64` might overflow.

## 8. Key Takeaways
- **Wraps are Deadly.** A single underflow can destroy a protocol's economy.
- **Explicit is Better.** Don't rely on compiler settings. Use `checked_` functions in the code to make safety explicit and readable.
- **Test Edges.** Always write tests that attempt to overflow/underflow your inputs.
