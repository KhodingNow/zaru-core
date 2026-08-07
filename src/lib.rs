// src/lib.rs
#![doc = include_str!("../README.md")]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]


//! # Zaru Core
//! 
//! A secure payment model engine with transaction state machine and cryptographic verification.
//!

//! ## Features
//! - Type-safe amount handling
//! - Transaction state machine (Unsigned → Signed → Verified)
//! - ED25519 cryptographic verification
//! - Wallet abstraction (Bank + Crypto)
//! - Optional Python bindings
//! 
//! ## Quick Start
//! ```rust
//! use zaru_core::{Amount, Transaction, TxId, WalletId, Keypair};
//! 
//! let from = WalletId::from("sender");
//! let to = WalletId::from_crypto("receiver");
//! let amount = Amount::new(1000).unwrap();
//! 
//! let tx = Transaction::<Unsigned>::new(
//!     TxId("txn_001".to_string()),
//!     from,
//!     to,
//!     amount,
//!     1
//! );
//! 
//! let keypair = Keypair::generate();
//! let signed = tx.sign(&keypair);
//! let verified = signed.verify().unwrap();
//! ```
// Module declarations

/// Error types for the payment engine.
///
/// Provides comprehensive error handling for all operations.
pub mod error;

/// Type-safe monetary mount handling.
///
/// Prevents negative values and ensures mathematical correctness.
pub mod amount;

/// Transaction state machine with cryptographic verification.
///
/// Implements the Unsigned -> Signed -> Verified lifecycle.
pub mod transaction;

/// Wallet abstraction supporting bank accounts and crypto addresses.
pub mod wallet;

/// Settlement layer for transaction finality.
pub mod settlement;

/// In-memory ledger implementation.
pub mod ledger;

/// Cryptographic primitives including ED25519 signatures.
pub mod crypto;

// Re-exports
pub use crate::amount::Amount;
pub use crate::transaction::{Transaction, TxId, Unsigned, Signed, Verified};
pub use crate::wallet::WalletId;
pub use crate::crypto::signature::Signature;
pub use crate::crypto::ed25519::Keypair;
pub use crate::crypto::verifier::CryptoVerifier;
pub use crate::error::{ZaruError, Result};

// Python bindings (conditional)
#[cfg(feature = "python-bindings")]
pub mod python;

// Test utilities
#[cfg(test)]
mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let amount = Amount::new(100).unwrap();
        assert_eq!(amount.value(), 100);
    }
}
