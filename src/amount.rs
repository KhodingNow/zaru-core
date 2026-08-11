// src/amount.rs

use core::ops::{Add, Sub};

/// A type-safe monetary amount that prevents negative values.
///
/// This is the core type for all monetary values in the system.
/// It ensures that amounts can never be negative at compile time.
///
/// # Examples
/// ```
/// use zaru_core::Amount;
///
/// let amount = Amount::new(1000).unwrap();
/// assert_eq!(amount.value(), 1000);
///
/// // Negative amounts are rejected
/// assert!(Amount::new(-100).is_err());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount(i128);

/// Errors that can occur when creating an 'Amount'.
#[derive(Debug)]
pub enum AmountError {
    /// The value was negative
    Negative,
}

impl Amount {
    /// Creates a new 'Amount' with the given value.
    ///
    /// # Errors
    /// Returns 'AmountError::Negative' if 'value' is negative.
    pub fn new(value: i128) -> Result<Self, AmountError> {
        if value < 0 {
            return Err(AmountError::Negative);
        }
        Ok(Self(value))
    }

    /// Returns a zero amount.
    pub fn zero() -> Self {
        Amount(0)
    }
    /// Returns the numeric value of the amount.
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
