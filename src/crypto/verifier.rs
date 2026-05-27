use ed25519_dalek::Verifier;

use crate::crypto::signature::Signature;
use crate::wallet::WalletId;

pub struct CryptoVerifier;

impl CryptoVerifier {
    pub fn verify(
        message: &[u8],
        signature: &Signature,
        expected_wallet: &WalletId,
    ) -> Result<(), &'static str> {

        // 1. cryptographic verification
        signature
            .public_key
            .verify(message, &signature.sig)
            .map_err(|_| "invalid signature")?;

        // 2. wallet ownership enforcement
        let derived_wallet =
            WalletId::from_ed25519(&signature.public_key);

        if &derived_wallet != expected_wallet {
            return Err("wallet does not match signing key");
        }

        Ok(())
    }
}
