use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;

use crate::amount::Amount;
use crate::settlement::traits::SettlementLayer;
use crate::settlement::types::{FailureReason, SettlementStatus};
use crate::transaction::{Transaction, TxId, Verified};

pub struct InMemorySettlement {
    store: Mutex<HashMap<TxId, SettlementStatus>>,
}

// ---- Constructor ----

impl InMemorySettlement {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

// ---- Trait Implementation ----

#[async_trait]
impl SettlementLayer for InMemorySettlement {
    type Error = std::io::Error;

    async fn submit(
        &self,
        tx: Transaction<Verified>,
    ) -> Result<TxId, Self::Error> {
        let mut store = self.store.lock().unwrap();

        // Idempotency: return existing if already submitted
        if store.contains_key(&tx.id) {
            return Ok(tx.id.clone());
        }

        // Insert as Pending
        store.insert(tx.id.clone(), SettlementStatus::Pending);

        Ok(tx.id)
    }

    async fn status(
        &self,
        tx_id: &TxId,
    ) -> Result<SettlementStatus, Self::Error> {
        let mut store = self.store.lock().unwrap();

        match store.get_mut(tx_id) {
            Some(current) => {
                // Simulate progression
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

    async fn await_finality(
        &self,
        tx_id: &TxId,
    ) -> Result<SettlementStatus, Self::Error> {
        loop {
            let status = self.status(tx_id).await?;

            if matches!(
                status,
                SettlementStatus::Finalized | SettlementStatus::Failed(_)
            ) {
                return Ok(status);
            }

            // Simulate delay
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    async fn estimate_fee(
        &self,
        _tx: &Transaction<Verified>,
    ) -> Result<Amount, Self::Error> {
        // Simple flat fee for now
        Ok(Amount::new(1).unwrap())
    }
}
