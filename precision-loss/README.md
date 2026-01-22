# Precision Loss (Integer Division)

## 1. High-Level Explanation
Precision loss in Solana programs typically occurs when performing integer arithmetic operations in the wrong order. Since the Solana Runtime (SVM) uses Rust's fixed-size integers (e.g., `u64`) and does not support floating-point numbers, division operations discard any fractional remainder (truncation).

If a program divides a number before multiplying it (e.g., `(a / b) * c`), any information in the remainder of `a / b` is irretrievably lost before the multiplication happens. This can lead to significant value loss, especially when `a < b`, where the result becomes `0`.

## 2. Why This Issue is Dangerous in Solana
In traditional finance, floating-point numbers (decimals) handle fractions. In Solana:
- **Token Economics:** Millions of dollars are often stored in `u64` integers (lamports). Small truncations can compound into massive arbitrage opportunities or fund leaks over time.
- **DeFi Mechanisms:** Protocols dealing with exchange rates, interest accrual, or staking rewards rely on precise ratios. "Rounding down to zero" can allow users to deposit funds without paying fees or claim rewards they didn't earn.
- **Irreversibility:** Once a transaction settles with a truncated value, the economic loss is permanent.

## 3. Root Cause Analysis
The root cause is the **Order of Operations** in integer arithmetic.
Mathematically, `(A * B) / C` is roughly equivalent to `(A / C) * B`.
Computationally, with integers:
- `(100 * 50) / 100 = 5000 / 100 = 50`
- `(100 / 100) * 50 = 1 * 50 = 50`
This looks fine.

**However, for small numbers:**
Let A = 99, B = 50, C = 100.
- **Correct (Multiply First):** `(99 * 50) / 100 = 4950 / 100 = 49` (Integer result of 49.5)
- **Vulnerable (Divide First):** `(99 / 100) * 50 = 0 * 50 = 0`

The vulnerabilities stem from the premature truncation in step 1.

## 4. Annotated Vulnerable Code Snippet
```rust
pub fn calculate_reward_bad(staked: u64, rate_bps: u64) -> Result<u64> {
    // VULNERABLE: Dividing before multiplying causes truncation.
    // If `staked` is less than 10,000, the result is 0, regardless of the rate.
    let step1 = staked.checked_div(10000).unwrap(); 
    let reward = step1.checked_mul(rate_bps).unwrap();
    Ok(reward)
}
```

## 5. Annotated Secure Code Snippet
```rust
pub fn calculate_reward_secure(staked: u64, rate_bps: u64) -> Result<u64> {
    // SECURE: Multiply first to preserve precision.
    // We increase the numerator size before dividing.
    // Use u128 to prevent overflow during the multiplication step if necessary.
    let numerator = (staked as u128).checked_mul(rate_bps as u128).unwrap();
    let reward = numerator.checked_div(10000).unwrap();
    
    // Cast back to u64 safely
    Ok(reward as u64)
}
```

## 6. Anchor vs Pinocchio Comparison

| Feature | Anchor | Pinocchio (Native) |
| :--- | :--- | :--- |
| **Input Parsing** | Automated via `DerivedAccounts` | Manual iteration of `account_info_iter` |
| **Arithmetic Safety** | Rust methods (`checked_mul`) or `u128` casting | Rust methods (`checked_mul`) or `u128` casting |
| **Error Handling** | `Constraint` or custom `error!` macros | `ProgramError` enum |
| **Boilerplate** | High (framework handles serialization) | Low (manual serialization required) |

**Note:** The arithmetic logic vulnerability remains identical requiring explicit handling in both frameworks. Anchor does not automatically prevent precision loss.

## 7. Security Patterns & Best Practices
1. **Multiplication Before Division:** Always sequence operations as `(a * b) / c`.
2. **Use `u128` for Intermediates:** When multiplying two `u64` values, cast to `u128` first to prevent overflow in the numerator (Solana has built-in `u128` support in BPF).
3. **Rounding Correction:** If rounding to nearest is needed, add half the divisor before dividing: `((a * b) + (c / 2)) / c`.
4. **Minimum Amounts:** Enforce minimum transaction amounts to ensure `amount > divisor` if dividing first is unavoidable (rare).
5. **Checked Math:** Always use `.checked_mul()` and `.checked_div()` or the `checked_arithmetic` macros to handle overflows/underflows gracefully.

## 8. Key Takeaways
- **Integer Math != Calculator Math.** 1/2 is 0, not 0.5.
- **Order Matters.** `Mul` then `Div` is the golden rule.
- **Unit Tests are Mandatory.** Test with inputs smaller than your divisor to catch these edge cases.
- **Audit Focus.** Search for `/` operators in codebase and verify the numerator was maximized prior to execute.
