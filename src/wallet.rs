use core::fmt;

use ed25519_dalek::VerifyingKey;

/// A bank account identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BankAccount(pub String);

impl From<&str> for BankAccount {
	fn from(value: &str) -> Self {
		BankAccount(value.to_string())
	}
}

/// A wallet that can be either a bank account or a cryptocurrency address.
/// 
/// # Examples
/// ```
/// use zaru_core::WalletId;
/// 
/// // Create a bank wallet
/// let bank = WalletId::from("Bank of America");
/// 
/// // Create a crypto wallet
/// let crypto = WalletId::from_crypto("0x1234567890abcdef");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WalletId {

    /// A trsditional bank account
    Bank(BankAccount),
    
    /// A cryptocurrency address
    Crypto(String),
}

impl WalletId {

    /// Creates a wallet from an ED25519 public key.
    /// 
    /// # Arguments
    /// * `pk` - The verifying (public) key
    /// 
    /// # Returns
    /// A crypto wallet with the hex-encoded public key as its address.
    pub fn from_ed25519(pk: &VerifyingKey) -> Self {
		WalletId::Crypto(hex::encode(pk.as_bytes()))
	}
}

// -------------------------
// DISPLAY
// -------------------------

impl fmt::Display for WalletId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletId::Bank(id) => write!(f, "{}",id.0),
            WalletId::Crypto(addr) => write!(f, "{}", addr),
        }
    }
}

// -------------------------
// FROM STR
// -------------------------

impl From<&str> for WalletId {
    fn from(value: &str) -> Self {
        WalletId::Bank(BankAccount(value.to_string()))
    }
}

// -------------------------
// FROM ED25519 PUBLIC KEY
// -------------------------

impl From<&VerifyingKey> for WalletId {
    fn from(key: &VerifyingKey) -> Self {
        WalletId::Crypto(hex::encode(key.as_bytes()))
    }
}
