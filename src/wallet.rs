#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SolanaAddress([u8; 32]);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BankAccount(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WalletId{
	Solana(SolanaAddress),
	Bank(BankAccount),
} 

impl From<&str> for BankAccount {
	fn from(value: &str) -> Self {
		BankAccount(value.to_string())
	}
}



impl From<&str> for WalletId {
	fn from(value: &str) -> Self {
		WalletId::Bank(value.into())
	}
}
