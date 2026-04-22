#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amount(i128);

#[derive(Debug)]
pub enum AmountError {
	NonPositive,
}

impl Amount {
	pub fn new(value: i128) -> Result<Self, AmountError> {
	if value <= 0 {
		return Err(AmountError::NonPositive);

		}
		Ok(Self(value))	


	}
	pub fn value(&self) -> i128 {
		self.0

	}
}
