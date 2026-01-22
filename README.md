# Solana Program Vulnerabilities Compendium

**A curated knowledge base of Solana-specific program security flaws, exploits, and remediations.**

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
    <summary>
        <strong>
            Arithmetic & Type Safety Vulnerabilities
        </strong>
    </summary>
    <p>
        <em>
            Arithmetic and type safety vulnerabilities involve unsafe mathematical operations and improper data type conversions that can lead to incorrect calculations or program state corruption, frequently occurring in unaudited code and ranking among the most common security audit findings due to Rust's permissive default behavior and the precision requirements of financial applications.
        </em>
    </p>

- [Precision Loss](https://github.com/ezedike-evan/solana-programs-vulnerabilities/tree/arithmetic-and-type-safety/precision-loss)

- [Integer Overflow / Underflow](https://github.com/ezedike-evan/solana-programs-vulnerabilities/tree/arithmetic-and-type-safety/integer-overflow-underflow)
</details>

## How to Navigate This Repository

The structure of this repository is designed for ease of use by judges and researchers.

1.  **Browse the Index**: Use the "Vulnerability Index" above to find a specific category of interest.
2.  **Read the Description**: Click the dropdown to read a high-level summary of the vulnerability class and why it matters.
3.  **Explore the Code**: Click on the specific vulnerability link or manually check out the branch using git:
    ```bash
    git checkout <branch-name>
    ```
4.  **Run Tests**: Follow the instructions in the branch-specific README (if available) or run `cargo test-bpf` to see the exploit in action.

## Licensing

- **Code**: Licensed under **MIT**. This ensures the code can be freely used, modified, and integrated into other projects, supporting open innovation in security tooling.
- **Documentation**: Licensed under **CC BY 4.0**. This facilitates broad dissemination of educational security material while requiring appropriate attribution.

**Disclaimer**: The code in the vulnerability folder is **intentionally insecure**. These contracts are for educational and testing purposes only. **Do not deploy to mainnet.**
