// src/python.rs
//! Python bindings for zaru-core payment engine
//! 
//! This module provides Python bindings using PyO3 for the zaru-core payment engine.
//! It exposes the following Python classes:
//! - `Amount`: Safe monetary value handling
//! - `WalletId`: Wallet abstraction (Bank + Crypto)
//! - `Transaction`: Payment transaction with verification
//! - `PaymentEngine`: Main engine for creating and managing transactions

#![allow(non_local_definitions)] // Suppress PyO3 macro warnings

use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyRuntimeError};
use serde_json::json;

use crate::{
    Amount, Transaction, TxId, WalletId, Unsigned, Verified,
    Keypair, CryptoVerifier,
};

/// Python wrapper for Amount
/// 
/// Represents a monetary amount with safe construction (prevents negative values).
/// 
/// Examples:
///     >>> from zaru_core import PaymentEngine
///     >>> engine = PaymentEngine()
///     >>> amount = engine.create_amount(1000)
///     >>> amount.value
///     1000
#[pyclass(name = "Amount")]
#[derive(Clone)]
pub struct PyAmount {
    inner: Amount,
}

#[pymethods]
impl PyAmount {
    /// Create a new Amount from an integer value
    /// 
    /// Raises ValueError if the amount is negative
    #[new]
    pub fn new(value: i128) -> PyResult<Self> {
        Amount::new(value)
            .map(|inner| PyAmount { inner })
            .map_err(|e| PyValueError::new_err(format!("Invalid amount: {:?}", e)))
    }

    /// Get the numeric value of the amount
    #[getter]
    pub fn value(&self) -> i128 {
        self.inner.value()
    }

    /// String representation for Python
    pub fn __repr__(&self) -> String {
        format!("Amount({})", self.inner.value())
    }

    /// Addition operator
    pub fn __add__(&self, other: &PyAmount) -> PyResult<PyAmount> {
        Ok(PyAmount {
            inner: self.inner.clone() + other.inner.clone(),
        })
    }

    /// Subtraction operator
    pub fn __sub__(&self, other: &PyAmount) -> PyResult<PyAmount> {
        Ok(PyAmount {
            inner: self.inner.clone() - other.inner.clone(),
        })
    }

    /// Convert to JSON string
    pub fn to_json(&self) -> String {
        json!({"value": self.inner.value()}).to_string()
    }
}

/// Python wrapper for WalletId
/// 
/// Represents a wallet that can be either a bank account or a cryptocurrency address.
/// 
/// Examples:
///     >>> from zaru_core import PaymentEngine
///     >>> engine = PaymentEngine()
///     >>> bank_wallet = engine.create_wallet("bank", "Bank of America")
///     >>> crypto_wallet = engine.create_wallet("crypto", "0x1234567890abcdef")
#[pyclass(name = "WalletId")]
#[derive(Clone)]
pub struct PyWalletId {
    inner: WalletId,
}

#[pymethods]
impl PyWalletId {
    /// Create a bank wallet from an account identifier
    #[staticmethod]
    pub fn from_bank(account: &str) -> Self {
        PyWalletId {
            inner: WalletId::Bank(crate::wallet::BankAccount(account.to_string())),
        }
    }

    /// Create a crypto wallet from an address
    #[staticmethod]
    pub fn from_crypto(address: &str) -> Self {
        PyWalletId {
            inner: WalletId::Crypto(address.to_string()),
        }
    }

    /// String representation for Python
    pub fn __repr__(&self) -> String {
        format!("WalletId('{}')", self.inner)
    }

    /// String conversion
    pub fn __str__(&self) -> String {
        self.inner.to_string()
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    /// Check if this is a bank wallet
    pub fn is_bank(&self) -> bool {
        matches!(self.inner, WalletId::Bank(_))
    }

    /// Check if this is a crypto wallet
    pub fn is_crypto(&self) -> bool {
        matches!(self.inner, WalletId::Crypto(_))
    }
}

/// Python wrapper for Transaction
/// 
/// Represents a payment transaction that has been signed and verified.
/// 
/// Examples:
///     >>> from zaru_core import PaymentEngine
///     >>> engine = PaymentEngine()
///     >>> tx = engine.create_transaction(
///     ...     id="txn_001",
///     ...     from_wallet=sender_wallet,
///     ...     to_wallet=receiver_wallet,
///     ...     amount=amount,
///     ...     nonce=1
///     ... )
///     >>> tx.verify_signature()
///     True
#[pyclass(name = "Transaction")]
pub struct PyTransaction {
    inner: Transaction<Verified>,
}

#[pymethods]
impl PyTransaction {
    /// Create a new unsigned transaction (internal use)
    /// 
    /// This is used by PaymentEngine to create transactions.
    /// Users should use PaymentEngine.create_transaction() instead.
    #[staticmethod]
    pub fn create_unsigned(
        id: &str,
        from_wallet: &PyWalletId,
        to_wallet: &PyWalletId,
        amount: &PyAmount,
        nonce: u64,
    ) -> PyResult<PyTransaction> {
        let tx_id = TxId(id.to_string());
        let unsigned = Transaction::<Unsigned>::new(
            tx_id,
            from_wallet.inner.clone(),
            to_wallet.inner.clone(),
            amount.inner.clone(),
            nonce,
        );

        // For demo, sign with a random keypair
        // In production, you'd want to pass the keypair
        let keypair = Keypair::generate();
        let signed = unsigned.sign(&keypair);

        match signed.verify() {
            Ok(verified) => Ok(PyTransaction { inner: verified }),
            Err(e) => Err(PyRuntimeError::new_err(e)),
        }
    }

    /// Get the transaction ID
    #[getter]
    pub fn id(&self) -> String {
        self.inner.id.0.clone()
    }

    /// Get the sender wallet
    #[getter]
    pub fn from_wallet(&self) -> PyWalletId {
        PyWalletId {
            inner: self.inner.from.clone(),
        }
    }

    /// Get the receiver wallet
    #[getter]
    pub fn to_wallet(&self) -> PyWalletId {
        PyWalletId {
            inner: self.inner.to.clone(),
        }
    }

    /// Get the transaction amount
    #[getter]
    pub fn amount(&self) -> PyAmount {
        PyAmount {
            inner: self.inner.amount.clone(),
        }
    }

    /// Get the transaction nonce
    #[getter]
    pub fn nonce(&self) -> u64 {
        self.inner.nonce
    }

    /// Get the transaction state
    #[getter]
    pub fn state(&self) -> String {
        "Verified".to_string()
    }

    /// Convert transaction to JSON
    pub fn to_json(&self) -> PyResult<String> {
        let data = json!({
            "id": self.inner.id.0,
            "from": self.inner.from.to_string(),
            "to": self.inner.to.to_string(),
            "amount": self.inner.amount.value(),
            "nonce": self.inner.nonce,
            "state": "Verified",
        });

        serde_json::to_string(&data)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Verify the transaction signature
    pub fn verify_signature(&self) -> PyResult<bool> {
        if let Some(sig) = &self.inner.signature {
            let message = format!(
                "{}:{}:{}:{}:{}",
                self.inner.id.0,
                self.inner.from,
                self.inner.to,
                self.inner.amount.value(),
                self.inner.nonce
            ).into_bytes();

            match CryptoVerifier::verify(&message, sig, &self.inner.from) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }
}

/// Main payment engine
/// 
/// The primary interface for creating and managing payment transactions.
/// 
/// Examples:
///     >>> from zaru_core import PaymentEngine
///     >>> engine = PaymentEngine()
///     >>> wallet = engine.create_wallet("bank", "My Bank")
///     >>> amount = engine.create_amount(1000)
///     >>> tx = engine.create_transaction(
///     ...     id="txn_001",
///     ...     from_wallet=wallet,
///     ...     to_wallet=wallet,
///     ...     amount=amount,
///     ...     nonce=1
///     ... )
#[pyclass(name = "PaymentEngine")]
pub struct PyPaymentEngine {
    // Engine state can be added here later
    // e.g., transaction_history: Vec<Transaction<Verified>>,
}

#[pymethods]
impl PyPaymentEngine {
    /// Create a new PaymentEngine instance
    #[new]
    pub fn new() -> Self {
        PyPaymentEngine {}
    }

    /// Create a new transaction
    /// 
    /// Args:
    ///     id: Unique transaction identifier
    ///     from_wallet: Sender wallet
    ///     to_wallet: Receiver wallet
    ///     amount: Transaction amount
    ///     nonce: Unique nonce for the transaction
    /// 
    /// Returns:
    ///     A signed and verified Transaction object
    #[pyo3(signature = (id, from_wallet, to_wallet, amount, nonce))]
    pub fn create_transaction(
        &self,
        id: &str,
        from_wallet: &PyWalletId,
        to_wallet: &PyWalletId,
        amount: &PyAmount,
        nonce: u64,
    ) -> PyResult<PyTransaction> {
        PyTransaction::create_unsigned(id, from_wallet, to_wallet, amount, nonce)
    }

    /// Create a new wallet
    /// 
    /// Args:
    ///     wallet_type: Either "bank" or "crypto"
    ///     identifier: Bank account or crypto address
    /// 
    /// Returns:
    ///     A WalletId object
    /// 
    /// Raises:
    ///     ValueError: If wallet_type is not "bank" or "crypto"
    pub fn create_wallet(&self, wallet_type: &str, identifier: &str) -> PyResult<PyWalletId> {
        match wallet_type.to_lowercase().as_str() {
            "bank" => Ok(PyWalletId::from_bank(identifier)),
            "crypto" => Ok(PyWalletId::from_crypto(identifier)),
            _ => Err(PyValueError::new_err(
                "Wallet type must be 'bank' or 'crypto'"
            )),
        }
    }

    /// Create a new Amount
    /// 
    /// Args:
    ///     value: The amount value (must be >= 0)
    /// 
    /// Returns:
    ///     An Amount object
    /// 
    /// Raises:
    ///     ValueError: If value is negative
    pub fn create_amount(&self, value: i128) -> PyResult<PyAmount> {
        PyAmount::new(value)
    }

    /// Verify a transaction's signature
    /// 
    /// Args:
    ///     tx: The transaction to verify
    /// 
    /// Returns:
    ///     True if the signature is valid, False otherwise
    pub fn verify_transaction(&self, tx: &PyTransaction) -> PyResult<bool> {
        tx.verify_signature()
    }
}

/// Export the Python module
/// 
/// This function defines what gets exported to Python.
#[pymodule]
pub fn zaru_core(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register classes
    m.add_class::<PyAmount>()?;
    m.add_class::<PyWalletId>()?;
    m.add_class::<PyTransaction>()?;
    m.add_class::<PyPaymentEngine>()?;

    // Add module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;

    // Utility functions
    #[pyfunction]
    fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    m.add_function(wrap_pyfunction!(version, m)?)?;

    #[pyfunction]
    fn generate_keypair() -> String {
        let keypair = Keypair::generate();
        hex::encode(keypair.verifying.as_bytes())
    }
    m.add_function(wrap_pyfunction!(generate_keypair, m)?)?;

    Ok(())
}
