## Arithmetic & Type Safety Vulnerabilities

Arithmetic and type safety vulnerabilities in Solana arise from the strict behavior of Rust's fixed-width integers and the absence of native floating-point support in the Solana virtual machine (SVM). These issues are critical because they can lead to inadvertently corrupted state, locked funds, or manipulated token economics given the deterministic nature of on-chain program execution. Developers must handle every arithmetic edge case manually to prevent logic errors that compromise program integrity.

### Integer Overflow / Underflow
Integer overflow occurs when an arithmetic operation results in a value exceeding the maximum representable limit for a given type, while underflow happens when a value drops below the minimum. In Rust (release mode), these operations typically wrap around rather than panic, potentially silently corrupting calculations. This is particularly dangerous in Solana programs managing token balances or voting weights, where a wrapped value can enable attackers to bypass limits or artificially inflate their holdings.

### Precision Loss (Integer Division)
Precision loss happens when integer division truncates the decimal portion of a result, leading to a discrepancy between the calculated and actual mathematical values. Since Solana programs do not support floating-point arithmetic, complex financial calculations (such as interest rates or fee distributions) must rely on integer math. Accumulation of these truncation errors over multiple operations can result in significant loss of funds or accounting imbalances within a smart contract.

### Rounding Direction Errors
Rounding direction errors occur when a program consistently rounds down (compounded floor) or rounds up (compounded ceiling) improperly during division or scaling operations. This issue typically manifests when calculating user rewards or exchange rates, where the chosen rounding method systematically disadvantages one detailed party over time. In Solana DeFi applications, exploiting predictable rounding behavior allows arbitrageurs to drain liquidity pools or extract value from infinitesimal discrepancies.

### Saturating Arithmetic Inaccuracy
Saturating arithmetic clamps values to the type's minimum or maximum bounds instead of wrapping or panicking when an overflow or underflow would occur. While this prevents runtime errors, it can introduce logical inaccuracies if the program assumes the calculation succeeded without modification. In the context of Solana state management, silently saturated values can lead to state desynchronization, where the on-chain data no longer accurately reflects the intended economic reality of the transaction.

### Casting Truncation
Casting truncation arises when converting a larger integer type to a narrower one (e.g., `u64` to `u32`), causing the higher-order bits to be discarded if the value exceeds the destination type's range. This can happen implicitly or explicitly during data serialization or index calculation. In Solana programs, truncation can lead to incorrect authorized amounts, invalid instruction data processing, or permission bypasses if a high-value parameter is effectively modulo-reduced to a smaller, valid range.
