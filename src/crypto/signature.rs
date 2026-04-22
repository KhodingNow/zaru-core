#[derive(Clone, Debug)]
pub struct Signature(Vec<u8>);

impl Signature {
	pub fn new(bytes: Vec<u8>) -> Self {
		Self(bytes)
	}
	
	pub fn as_bytes(&self) -> &[u8] {
	
		&self.0
	}
}
