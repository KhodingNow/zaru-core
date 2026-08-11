//! Test utilities for zaru-core
//!
//! This module contains helper functions and types for testing.

#![cfg(test)]
use crate::amount::Amount;
use crate::transaction::{Transaction, TxId, Unsigned};
use crate::wallet::WalletId;

/// Creates a test wallet for testing purposes
#[cfg(test)]
pub fn test_wallet() -> WalletId {
    WalletId::from("test_wallet")
}

/// Creates a test transaction for testing purposes
#[cfg(test)]
pub fn test_transaction() -> Transaction<Unsigned> {
    let from = WalletId::from("test_sender");
    let to = WalletId::from("test_receiver");
    let amount = Amount::new(100).unwrap();

    Transaction::<Unsigned>::new(TxId("test_tx".to_string()), from, to, amount, 0)
}

/// Creates a test amount for testing purposes
#[cfg(test)]
pub fn test_amount() -> Amount {
    Amount::new(100).unwrap()
}
