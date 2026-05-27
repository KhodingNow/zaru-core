use async_trait::async_trait;

use crate::amount::Amount;
use crate::settlement::types::SettlementStatus;
use crate::transaction::{Transaction, TxId, Verified};

#[async_trait]
pub trait SettlementLayer {
    type Error;

    async fn submit(
        &self,
        tx: Transaction<Verified>,
    ) -> Result<TxId, Self::Error>;

    async fn status(
        &self,
        tx_id: &TxId,
    ) -> Result<SettlementStatus, Self::Error>;

    async fn await_finality(
        &self,
        tx_id: &TxId,
    ) -> Result<SettlementStatus, Self::Error>;

    async fn estimate_fee(
        &self,
        tx: &Transaction<Verified>,
    ) -> Result<Amount, Self::Error>;
}
