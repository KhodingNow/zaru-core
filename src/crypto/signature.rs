use ed25519_dalek::{Signature as DalekSignature, VerifyingKey};

#[derive(Clone, Debug)]
pub struct Signature {
    pub sig: DalekSignature,
    pub public_key: VerifyingKey,
}

impl Signature {
    pub fn new(sig: DalekSignature, public_key: VerifyingKey) -> Self {
        Self { sig, public_key }
    }
}
