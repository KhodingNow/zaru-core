// src/error.rs
use std::fmt;
use thiserror::Error;

/// All possible errors that can occur in the payment engine.
#[derive(Error, Debug)]
pub enum ZaruError {
     /// The amount is invalid (e.g., negative value).
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    /// Transaction validation failed.
    #[error("Transaction error: {0}")]
    TransactionError(String),

    /// Cryptographic signature error.    
    #[error("Signature error: {0}")]
    SignatureError(String),

    /// Verification of a signature failed.   
    #[error("Verification error: {0}")]
    VerificationError(String),

    /// Wallet operation failed.    
    #[error("Wallet error: {0}")]
    WalletError(String),
    
    /// Settlement operation failed.
    #[error("Settlement error: {0}")]
    SettlementError(String),
    
    /// Cryptographic operation failed.
    #[error("Crypto error: {0}")]
    CryptoError(String),
    
    /// Serialization or de-serialization failed.
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// I/O operation failed.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}


/// Result type alias for the payment engine.
pub type Result<T> = std::result::Result<T, ZaruError>;
