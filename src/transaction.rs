use std::marker::PhantomData;


use crate::amount::Amount;
use crate::crypto::verifier::CryptoVerifier;
use crate::crypto::ed25519::Keypair;
use crate::crypto::signature::Signature;
use crate::wallet::WalletId;

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

impl<State> Transaction<State> {
    fn signing_bytes(&self) -> Vec<u8> {
        format!(
            "{}:{}:{}:{}:{}",
            self.id.0,
            self.from,
            self.to,
            self.amount.value(),
            self.nonce
        )
        .into_bytes()
    }
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

    pub fn sign(self, keypair: &Keypair) -> Transaction<Signed> {
        let message = self.signing_bytes();

        let sig = keypair.sign(&message);

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
            signature: Some(Signature {
                sig,
                public_key: keypair.verifying,
            }),
            _state: PhantomData,
        }
    }
}

// ---- SIGNED ----

impl Transaction<Signed> {
    pub fn verify(self) -> Result<Transaction<Verified>, &'static str> {
        let signature = self
            .signature
            .as_ref()
            .ok_or("missing signature")?;

        let message = self.signing_bytes();

        CryptoVerifier::verify(
            &message,
            signature,
            &self.from,
        )?;

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
