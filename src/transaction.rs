use std::marker::PhantomData;

use crate::amount::Amount;
use crate::wallet::WalletId;
use crate::crypto::signature::Signature;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TxId(pub String);

// ---- STATE MARKERS ----

#[derive(Clone, Debug)]
pub struct Unsigned;

#[derive(Clone, Debug)]
pub struct Signed;

#[derive(Clone, Debug)]
pub struct Verified;

// ---- TRANSACTION ----

#[derive(Clone, Debug)]
pub struct Transaction<State> {
    pub id: TxId,
    pub from: WalletId,
    pub to: WalletId,
    pub amount: Amount,
    pub nonce: u64,
    pub signature: Option<Signature>,
    _state: PhantomData<State>,
}


// ---- UNSIGNED ----

impl Transaction<Unsigned> {
    pub fn new(
        id: TxId,
        from: WalletId,
        to: WalletId,
        amount: Amount,
        nonce: u64,
    ) -> Self {
		assert!(amount.value() > 0, "transaction amount must be positive");	

        Self {
            id,
            from,
            to,
            amount,
            nonce,
            signature: None,
            _state: PhantomData,
        }
    }

    pub fn sign(self, sig: Signature) -> Transaction<Signed> {
        let Transaction {
            id,
            from,
            to,
            amount,
            nonce,
            ..
        } = self;

        Transaction {
            id,
            from,
            to,
            amount,
            nonce,
            signature: Some(sig),
            _state: PhantomData,
        }
    }
}

// ---- SIGNED ----

impl Transaction<Signed> {
    pub fn verify(self) -> Result<Transaction<Verified>, &'static str> {
        if self.signature.is_none() {
            return Err("missing signature");
        }

        let Transaction {
            id,
            from,
            to,
            amount,
            nonce,
            signature,
            ..
        } = self;

        Ok(Transaction {
            id,
            from,
            to,
            amount,
            nonce,
            signature,
            _state: PhantomData,
        })
    }
}
