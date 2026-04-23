use std::collections::HashMap;
use std::sync::Mutex;

use crate::amount::Amount;
use crate::wallet::WalletId;

#[derive(Debug)]
pub struct LedgerState {
	balances: Mutex<HashMap<WalletId, Amount>>,
}

impl LedgerState {
	pub fn new() -> Self {
		Self {
			balances: Mutex::new(HashMap::new()),
		}
	}

	// Initialize / fund account ---
	pub fn deposit(&self, wallet: WalletId, amount: Amount) {
		let mut balances = self.balances.lock().unwrap();

		balances
			.get(wallet)
			.cloned()
			.unwrap_or_else(Amount::zero)
	}
	
	// --- core: Apply transfer atomically ---
	pub fn apply_transfer(
		&self,
		from: &WalletId,
		to: &WalletId,
		amount: &Amount,
	) -> Result<(), &'static str> {
		let mut balances = self.balances.lock().unwrap();

		let from_balance = balances
			.get(from)
			.cloned()
			.unwrap_or_else(Amount::zero);

	// -- Check funds---
	if from_balance < *amount {
			return Err("insufficient funds");
	}
	
	// -- Debit sender --
	balances.insert(from.clone(), from_balance - amount.clone());

	// -- Credit receiver --
	let to_balance = balances
		.get(to)
		.clone()
		.unwrap_or_else(Amount::zero);

	balances.insert(to.clone(), to_balance + amount.clone());

	OK(())
	
	}
}
