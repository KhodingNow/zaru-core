use std::marker::PhantomData;

use std::format;

use crate::amount::Amount;
use crate::crypto::ed25519::Keypair;
use crate::crypto::signature::Signature;
use crate::crypto::verifier::CryptoVerifier;
use crate::wallet::WalletId;

/// A unique identifier for a transaction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TxId(pub String);

// ---- STATE MARKERS ----

/// Marker type for an unsigned transaction.
#[derive(Clone, Debug)]
pub struct Unsigned;

/// Marker type for a signed transaction.
#[derive(Clone, Debug)]
pub struct Signed;

/// Marker type for a verified transaction.
#[derive(Clone, Debug)]
pub struct Verified;

// ---- TRANSACTION ----

/// A payment transaction with a state machine.
///
/// The state machine ensures that transactions follow a valid lifecycle:
/// - `Transaction<Unsigned>`: Created but not signed
/// - `Transaction<Signed>`: Signed but not verified
/// - `Transaction<Verified>`: Verified and ready for settlement
#[derive(Clone, Debug)]
pub struct Transaction<State> {
    /// Unique transaction identifier
    pub id: TxId,
    /// Sender's wallet
    pub from: WalletId,
    /// Receiver's wallet
    pub to: WalletId,
    /// Transaction amount
    pub amount: Amount,
    /// Anti-replay nonce
    pub nonce: u64,
    /// Cryptographic signature (present for Signed/Verified states)
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
    /// Creates a new  unsigned transaction.
    ///
    /// # Arguments
    /// * 'id' - Unique transaction identifier
    /// * 'from' - Sender's wallet
    /// * 'to' - Receiver's wallet
    /// * 'amount' - Transaction amount
    /// * 'nonce' - Anti-replay nonce (must be unique per sender)
    pub fn new(id: TxId, from: WalletId, to: WalletId, amount: Amount, nonce: u64) -> Self {
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

    /// Signs the transaction with a keypair.
    ///
    /// # Arguments
    /// * `keypair` - The signer's cryptographic keypair
    ///
    /// # Returns
    /// A `Transaction<Signed>` containing the signature.
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
    /// Verifies the transaction signature.
    ///
    /// # Returns
    /// A `Transaction<Verified>` if the signature is valid.
    ///
    /// # Errors
    /// Returns an error string if the signature is invalid or missing.    
    pub fn verify(self) -> Result<Transaction<Verified>, &'static str> {
        let signature = self.signature.as_ref().ok_or("missing signature")?;

        let message = self.signing_bytes();

        CryptoVerifier::verify(&message, signature, &self.from)?;

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
