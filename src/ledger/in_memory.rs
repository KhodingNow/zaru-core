use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::amount::Amount;
use crate::settlement::traits::SettlementLayer;
use crate::settlement::types::{FailureReason, SettlementStatus};
use crate::transaction::{Transaction, TxId, Verified};
use crate::wallet::WalletId;

#[derive(Debug)]
pub struct InMemorySettlement {
    store: Mutex<HashMap<TxId, SettlementStatus>>,
    nonces: Mutex<HashMap<WalletId, u64>>,
    balances: Mutex<HashMap<WalletId, Amount>>,
}

impl Default for InMemorySettlement {
    fn default() -> Self {
        Self::new()
    }
}

// ---- Constructor ----

impl InMemorySettlement {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
            nonces: Mutex::new(HashMap::new()),
            balances: Mutex::new(HashMap::new()),
        }
    }

    // ---- Ledger Helpers ----

    pub fn deposit(&self, wallet: WalletId, amount: Amount) {
        let mut balances = self.balances.lock().unwrap();

        let entry = balances.entry(wallet).or_insert(Amount::new(0).unwrap());

        *entry = Amount::new(entry.value() + amount.value()).unwrap();
    }

    pub fn balance(&self, wallet: &WalletId) -> Option<Amount> {
        let balances = self.balances.lock().unwrap();
        balances.get(wallet).cloned()
    }

    pub fn transaction(&self, tx_id: &TxId) -> Option<SettlementStatus> {
        let store = self.store.lock().unwrap();
        store.get(tx_id).cloned()
    }
}

// ---- Trait Implementation ----

#[async_trait]
impl SettlementLayer for InMemorySettlement {
    type Error = std::io::Error;

    async fn submit(&self, tx: Transaction<Verified>) -> Result<TxId, Self::Error> {
        // -------------------------
        // LOCK ORDER (IMPORTANT)
        // -------------------------
        let mut store = self.store.lock().unwrap();
        let mut nonces = self.nonces.lock().unwrap();
        let mut balances = self.balances.lock().unwrap();

        // -------------------------
        // 1. IDEMPOTENCY FIRST
        // -------------------------
        if store.get(&tx.id).is_some() {
            return Ok(tx.id);
        }

        // -------------------------
        // 2. NONCE VALIDATION
        // -------------------------
        let sender = tx.from.clone();
        let incoming_nonce = tx.nonce;

        if let Some(last_nonce) = nonces.get(&sender)
            && incoming_nonce <= *last_nonce
        {
            return Err(std::io::Error::other("nonce replay detected"));
        }

        // -------------------------
        // 3. BALANCE CHECK
        // -------------------------
        let sender_balance = balances
            .get(&tx.from)
            .cloned()
            .unwrap_or_else(|| Amount::new(0).unwrap());

        if sender_balance.value() < tx.amount.value() {
            return Err(std::io::Error::other("insufficient funds"));
        }

        // -------------------------
        // 4. APPLY TRANSFER (ATOMIC)
        // -------------------------

        // Deduct sender
        balances.insert(
            tx.from.clone(),
            Amount::new(sender_balance.value() - tx.amount.value()).unwrap(),
        );

        // Credit receiver
        let receiver_balance = balances
            .get(&tx.to)
            .cloned()
            .unwrap_or_else(|| Amount::new(0).unwrap());

        balances.insert(
            tx.to.clone(),
            Amount::new(receiver_balance.value() + tx.amount.value()).unwrap(),
        );

        // -------------------------
        // 5. STORE TRANSACTION
        // -------------------------
        store.insert(tx.id.clone(), SettlementStatus::Pending);

        // -------------------------
        // 6. UPDATE NONCE
        // -------------------------
        nonces.insert(sender, incoming_nonce);

        Ok(tx.id)
    }

    async fn status(&self, tx_id: &TxId) -> Result<SettlementStatus, Self::Error> {
        let mut store = self.store.lock().unwrap();

        match store.get_mut(tx_id) {
            Some(current) => {
                *current = match *current {
                    SettlementStatus::Pending => SettlementStatus::Confirmed,
                    SettlementStatus::Confirmed => SettlementStatus::Finalized,
                    SettlementStatus::Finalized => SettlementStatus::Finalized,
                    SettlementStatus::Failed(ref reason) => {
                        SettlementStatus::Failed(reason.clone())
                    }
                };

                Ok(current.clone())
            }
            None => Ok(SettlementStatus::Failed(FailureReason::Unknown)),
        }
    }

    async fn await_finality(&self, tx_id: &TxId) -> Result<SettlementStatus, Self::Error> {
        loop {
            let status = self.status(tx_id).await?;

            if matches!(
                status,
                SettlementStatus::Finalized | SettlementStatus::Failed(_)
            ) {
                return Ok(status);
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    async fn estimate_fee(&self, _tx: &Transaction<Verified>) -> Result<Amount, Self::Error> {
        Ok(Amount::new(1).unwrap())
    }
}
