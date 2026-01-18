# Solana Smart Contract Vulnerabilities

This repository serves as a curated knowledge base of common and advanced security vulnerabilities found in Solana smart contracts. It is designed as an educational resource for developers, auditors, and security researchers to understand, identify, and prevent security pitfalls specific to the Solana execution model and program architecture.

## Repository Structure

The `main` branch of this repository intentionally contains only the documentation and licensing information. To minimize confusion and ensure separation of concerns, the vulnerability implementations are isolated in separate branches.

Each specific vulnerability branch contains:
- A vulnerable Rust contract demonstrating the flaw
- A secure Rust contract implementing the fix
- Integration tests demonstrating the exploit and verifying the unified fix

## Licensing

- All code samples and software implementations are licensed under the **MIT License**.
- All documentation, research notes, and educational content are licensed under **Creative Commons Attribution 4.0 International (CC BY 4.0)**.

## Vulnerability Index

<details>
<summary>Account Model & Ownership Errors</summary>

- [Missing Owner Check](tree/account-owner-check) (branch: `account-owner-check`)
- [Account Confusion / Type Confusion](tree/account-confusion) (branch: `account-confusion`)
- [Account Data Matching](tree/account-data-matching) (branch: `account-data-matching`)
- [Closing Accounts & Lamport Leaks](tree/closing-accounts) (branch: `closing-accounts`)

</details>

<details>
<summary>CPI & Cross-Program Invocation Issues</summary>

- [Unchecked CPI Accounts](tree/unchecked-cpi) (branch: `unchecked-cpi`)
- [Privilege Escalation via CPI](tree/signed-cpi-privilege) (branch: `signed-cpi-privilege`)
- [Program ID Spoofing](tree/program-id-spoofing) (branch: `program-id-spoofing`)

</details>

<details>
<summary>PDA & Seed Misuse</summary>

- [PDA Bump Seed Collision](tree/pda-bump-collision) (branch: `pda-bump-collision`)
- [Insecure Seed Derivation](tree/insecure-seeds) (branch: `insecure-seeds`)
- [PDA Sharing](tree/pda-sharing) (branch: `pda-sharing`)

</details>

<details>
<summary>Arithmetic & Logic Errors</summary>

- [Integer Overflow/Underflow](tree/integer-overflow) (branch: `integer-overflow`)
- [Precision Loss in Calculations](tree/precision-loss) (branch: `precision-loss`)
- [Incorrect Deserialize Checks](tree/incorrect-deserialize) (branch: `incorrect-deserialize`)

</details>

<details>
<summary>Authorization & Signer Issues</summary>

- [Missing Signer Check](tree/missing-signer) (branch: `missing-signer`)
- [Signature Replay Attacks](tree/signature-replay) (branch: `signature-replay`)
- [Authority Validation Bypass](tree/authority-bypass) (branch: `authority-bypass`)

</details>

<details>
<summary>State Initialization & Upgrade Risks</summary>

- [Re-initialization Attack](tree/reinitialization-attack) (branch: `reinitialization-attack`)
- [Duplicate Account Initialization](tree/duplicate-init) (branch: `duplicate-init`)
- [Upgrade Authority Management](tree/upgrade-authority) (branch: `upgrade-authority`)

</details>

## How to Use This Repository

To explore a specific vulnerability:

1. Identify the vulnerability category in the Index above.
2. Switch to the corresponding branch listed (e.g., `git checkout account-confusion`).
3. Review the `vulnerable` and `secure` program implementations.
4. Run the provided tests to observe the exploit and the remediation.

This repository is intended for:
- **Developers** looking to write secure Solana programs.
- **Auditors** referencing common attack vectors.
- **Learners** studying high-assurance engineering.

**Warning**: The code in the vulnerable examples is intentionally insecure. Do not deploy these contracts to a production environment.

## Contribution Guidelines

We welcome contributions from the community. To maintain the quality of this knowledge base:

- **Quality**: Submissions must include both vulnerable and fixed code paths.
- **Testing**: Proof-of-concept tests demonstrating the exploit are required.
- **Clarity**: Explanations must be technically accurate and concise.
- **Effort**: Low-effort submissions or theoretical issues without practical demonstration will not be merged.
