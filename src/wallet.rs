use std::fmt;

use ed25519_dalek::VerifyingKey;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BankAccount(pub String);

impl From<&str> for BankAccount {
	fn from(value: &str) -> Self {
		BankAccount(value.to_string())
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WalletId {
    Bank(BankAccount),
    Crypto(String),
}

impl WalletId {
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
