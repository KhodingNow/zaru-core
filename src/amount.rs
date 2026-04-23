use std::ops::{Add, Sub};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i128);

#[derive(Debug)]
pub enum AmountError {
	Negative,
}

impl Amount {

	// --- Constructor: allow zero ---
	pub fn new(value: i128) -> Result<Self, AmountError> {
	if value < 0 {
		return Err(AmountError::Negative);

		}
		Ok(Self(value))	


	}
	// --- Safe constructor for known-good values ---
	pub fn zero() -> Self {
		Amount(0)
	}
	
	pub fn value(&self) -> i128 {
		self.0

	}
}

impl Sub for Amount {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Amount(self.0 - rhs.0) 
	} 
}

impl Add for Amount {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Amount(self.0 + rhs.0)
	}
}
