use ed25519_dalek::{Signature, Signer, Verifier, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

// --KeyPair --

pub struct Keypair {
	pub signing: SigningKey,
	pub verifying: VerifyingKey,
}

impl Keypair {
	pub fn generate() -> Self {
		let signing = SigningKey::generate(&mut OsRng);
		let verifying = signing.verifying_key();

		Self { signing, verifying }

	}

	pub fn sign(&self, message: &[u8]) -> Signature {
		self.signing.sign(message)
	
	}
}	

// -- VERIFY --

pub fn verify(
	public_key: &VerifyingKey,
	message: &[u8],
	signature: &Signature,

) -> bool {
	public_key.verify(message, signature).is_ok()
}	
