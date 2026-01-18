# Solana Smart Contract Vulnerabilities Compendium

**A curated knowledge base of Solana-specific smart contract security flaws, exploits, and remediations.**

This repository serves as a comprehensive reference for real-world vulnerabilities found in the Solana execution model. Each documented vulnerability is isolated in its own branch and includes:
- A **vulnerable Rust contract** demonstrating the flaw.
- A **secure / patched version** implementing the fix.
- **Integration tests** that prove the exploit and verify the remediation.

The main branch contains only this documentation to maintain a clean, audit-friendly structure and prevent accidental deployment of insecure code.

## Repository Philosophy

This compendium is structured to serve **auditors**, **security researchers**, **Solana developers**, and **competition judges**.

By isolating each vulnerability into a dedicated branch, we ensure:
1.  **Clarity**: Researchers can focus on one specific attack vector at a time without noise from a monolithic codebase.
2.  **Safety**: Vulnerable implementations are strictly segregated, eliminating the risk of accidental deployment to mainnet.
3.  **verifiability**: Each branch allows for independent testing and verification of both the exploit and the patch.

This approach mirrors the rigor expected in high-assurance engineering and auditing environments.

## Vulnerability Index

<details>
<summary><strong>Account Validation Errors</strong></summary>

<p><em>This category covers vulnerabilities arising from insufficient validation of account input data. In Solana, programs must manually verify that passed accounts are of the expected type, owned by the expected program, and correctly initialized. Failure to validate these properties allows attackers to inject malicious accounts, leading to unauthorized state modification or arbitrary code execution paths.</em></p>

- [Missing Owner Check](tree/account-owner-check)
- [Account Confusion / Type Confusion](tree/account-confusion)
- [Account Data Matching](tree/account-data-matching)
- [Arbitrary Account Injection](tree/arbitrary-account-injection)

</details>

<details>
<summary><strong>Authority & Signer Misuse</strong></summary>

<p><em>These vulnerabilities involve defects in how a program verifies transaction authorities and signatures. Solana programs must explicitly check if a required account has signed the transaction before performing sensitive actions like transferring funds or updating state. Neglecting these checks enables attackers to execute privileged instructions without proper authorization, resulting in theft of funds or protocol takeover.</em></p>

- [Missing Signer Check](tree/missing-signer)
- [Signature Replay Attacks](tree/signature-replay)
- [Authority Validation Bypass](tree/authority-bypass)
- [Instruction Introspection Abuse](tree/instruction-introspection)

</details>

<details>
<summary><strong>PDA Derivation & Seed Collisions</strong></summary>

<p><em>Program Derived Addresses (PDAs) allow programs to sign for accounts, but they require strict deterministic derivation. Vulnerabilities here stem from insecure seed selection, missing bump seed validation, or allowing user-controlled inputs to collide with reserved addresses. Exploitation often results in attackers controlling program-owned accounts or overwriting critical protocol state.</em></p>

- [PDA Bump Seed Collision](tree/pda-bump-collision)
- [Insecure Seed Derivation](tree/insecure-seeds)
- [PDA Sharing](tree/pda-sharing)
- [Map Key Collision](tree/map-key-collision)

</details>

<details>
<summary><strong>Reentrancy & CPI Abuse</strong></summary>

<p><em>Cross-Program Invocations (CPI) allow composability but introduce complexity regarding trust and control flow. This category examines risks such as passing arbitrary programs to CPI calls without verification or handling state inconsistently across invocations. While Solana's runtime prevents traditional reentrancy, logical reentrancy and privilege escalation via signed CPIs remain critical threats.</em></p>

- [Unchecked CPI Accounts](tree/unchecked-cpi)
- [Privilege Escalation via CPI](tree/signed-cpi-privilege)
- [Program ID Spoofing](tree/program-id-spoofing)
- [Read-Only Reentrancy](tree/read-only-reentrancy)

</details>

<details>
<summary><strong>Integer & Arithmetic Errors</strong></summary>

<p><em>Mathematical operations in Rust are generally safe, but smart contracts often handle high-value assets where precision and boundary conditions are critical. This section documents issues like integer overflow/underflow in unchecked blocks and precision loss during division. These errors can cause funds to be locked permanently or allow attackers to manipulate exchange rates and balance calculations.</em></p>

- [Integer Overflow/Underflow](tree/integer-overflow)
- [Precision Loss in Calculations](tree/precision-loss)
- [Incorrect Deserialize Checks](tree/incorrect-deserialize)
- [Casting Truncation](tree/casting-truncation)

</details>

<details>
<summary><strong>Rent, Lamports & Balance Handling</strong></summary>

<p><em>Solana's rent model requires accounts to maintain a minimum balance to remain rent-exempt, and closing accounts requires transferring all lamports out. Vulnerabilities occur when programs fail to check rent exemption status or incorrectly calculate balance transfers during account closure. These issues can lead to "dust" accounts that clutter the state or loss of user funds during withdrawal sequences.</em></p>

- [Closing Accounts & Lamport Leaks](tree/closing-accounts)
- [Rent Exemption Bypass](tree/rent-exemption)
- [Insufficient Funds Handling](tree/insufficient-funds)

</details>

<details>
<summary><strong>Instruction Data & Serialization Bugs</strong></summary>

<p><em>Secure deserialization of instruction data is the first line of defense for any smart contract. Vulnerabilities in this category involve trusting the length or format of input data without validation, or discrepancies between serialization standards (e.g., Borsh vs. manual slicing). Malformed inputs can trigger unexpected program behavior, bypass logic gates, or cause denial of service.</em></p>

- [Incorrect Deserialization](tree/incorrect-deserialization)
- [Enum Type Confusion](tree/enum-type-confusion)
- [Buffer Overreads](tree/buffer-overread)

</details>

## How to Navigate This Repository

The structure of this repository is designed for ease of use by judges and researchers.

1.  **Browse the Index**: Use the "Vulnerability Index" above to find a specific category of interest.
2.  **Read the Description**: Click the dropdown to read a high-level summary of the vulnerability class and why it matters.
3.  **Explore the Code**: Click on the specific vulnerability link (e.g., `Missing Owner Check`) or manually check out the branch using git:
    ```bash
    git checkout <branch-name>
    ```
4.  **Run Tests**: Follow the instructions in the branch-specific README (if available) or run `cargo test-bpf` to see the exploit in action.

## Licensing

- **Code**: Licensed under **MIT**. This ensures the code can be freely used, modified, and integrated into other projects, supporting open innovation in security tooling.
- **Documentation**: Licensed under **CC BY 4.0**. This facilitates broad dissemination of educational security material while requiring appropriate attribution.

**Disclaimer**: The code in the vulnerability branches is **intentionally insecure**. These contracts are for educational and testing purposes only. **Do not deploy to mainnet.**
