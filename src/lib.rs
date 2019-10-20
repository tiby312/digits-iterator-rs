//! This crate adds an extension method to the integers that permits
//! to iterate over their digits.
//!
//! Note that the signed integers will be casted to the corresponding unsigned
//! integers. Do not use this iterator with negative signed integers unless you
//! *really* want to iterate over the digits of the complement.
//!
//! To use this extension, add the crate and import its content:
//!
//! ```
//! extern crate digits_iterator;
//! use digits_iterator::*;
//! ```
//!
//! # Examples
//!
//! ```rust
//! use digits_iterator::*;
//!
//! let digits: Vec<_> = 2018_u32.digits().collect();
//! assert_eq!(digits[..], [2, 0, 1, 8]);
//!
//! let digits: Vec<_> = 0b101010.digits_with_base(2).collect();
//! assert_eq!(digits[..], [1_u8, 0, 1, 0, 1, 0]);
//! ```


#[cfg(test)]
mod tests;

mod int;
use int::Int;

mod digits;
pub use digits::Digits;

/// Adds the extension methods to iterate over the digits of integers.
pub trait DigitsExtension: Int {
    /// Iterates over the digits of self in decimal base.
    ///
    /// # Example
    ///
    /// ```
    /// use digits_iterator::*;
    ///
    /// let n = 2018_u32;
    /// let digits: Vec<_> = n.digits().collect();
    ///
    /// assert_eq!(digits, [2, 0, 1, 8]);
    /// ```
    fn digits(self) -> Digits<Self> {
        Digits::new(self)
    }

    /// Iterates over the digits of self in a base between 2 and 36.
    ///
    /// # Example
    ///
    /// ```
    /// use digits_iterator::*;
    ///
    /// let n = 0b100101_u32;
    /// let digits: Vec<_> = n.digits_with_base(2).collect();
    ///
    /// assert_eq!(digits, [1, 0, 0, 1, 0, 1]);
    /// ```
    fn digits_with_base(self, base: u32) -> Digits<Self> {
        Digits::with_base(self, base)
    }
}

impl<T> DigitsExtension for T
where
    T: Int,
{
}
