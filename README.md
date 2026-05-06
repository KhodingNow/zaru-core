Zaru Core is a Rust-based transaction processing engine designed to model secure, deterministic state transitions in distributed systems.

The project explores how financial-grade systems can enforce:

Transaction integrity
Cryptographic authenticity
Safe state evolution
Fault-tolerant processing

Built with Tokio for async execution and ed25519 for cryptographic verification, Zaru Core demonstrates how to design systems that remain predictable, verifiable, and resilient under load.

Fintech System Alignment

This project reflects core principles used in payment platforms:

Idempotent transaction flows (preventing duplicate execution)
Deterministic processing (consistent outcomes under concurrency)
Ledger-style state management
Verification-first architecture (reject invalid transactions early)

These patterns are critical in high-volume payment systems where correctness and reliability are non-negotiable.
