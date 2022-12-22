//! # cik
//!
//! `cik` provides a `CIK` type for working with validated (syntax only)
//! [Central Index Keys (CIKs)](https://www.sec.gov/page/edgar-glossary#cik) as defined in the
//! [U.S. Securities and Exchange Commission's (SEC)](https://www.sec.gov)
//! [Electronic Data Gathering, Analysis, and Retrieval system (EDGAR)](https://www.sec.gov/edgar.shtml)
//! documentation.
//!
//! A CIK is a number of up to 10 digits length. They are sometimes rendered as strings with or without leading zeros, and
//! sometimes are represented as integers of a sufficient number of bits to represent a 10-digit number (typically 64 bits
//! because the maximum CIK value is 9,999,999,999 and the maximum value of a signed 32-bit integer is only 2,147,483,64;
//! and the maximum value of an unsigned 32-bit integer is still to low at 4,294,967,295).
//!
//! As of 2022-05-29 the "Company Facts" data set's minimum CIK value is 1,750 and the maximum is 1,923,807. See
//! the "Bulk data" section on the SEC's
//! [EDGAR Application Programming Interfaces](https://www.sec.gov/edgar/sec-api-documentation) page for more information on
//! this data set. Such values fit comfortably in 7 decimal digits or 32 bits (signed or unsigned), so you might encounter
//! CIKs stored in values of less than 64 bits.
//!
//! Nonetheless, this library uses 64-bit values to ensure full conformance with the CIK definition of up to 10 decimal
//! digits.
//!
//! ## Related crates
//!
//! This crate is part of the Financial Identifiers series:
//!
//! * [CIK](https://crates.io/crates/cik): Central Index Key (SEC EDGAR)
//! * [CUSIP](https://crates.io/crates/cusip): Committee on Uniform Security Identification Procedures (ANSI X9.6-2020)
//! * [ISIN](https://crates.io/crates/isin): International Securities Identification Number (ISO 6166:2021)
//! * [LEI](https://crates.io/crates/lei): Legal Entity Identifier (ISO 17442:2020)
//!

use std::fmt;
use std::str::FromStr;

pub mod error;
pub use error::CIKError;

/// Parse a string to a valid CIK or an error message, requiring the string to already be only
/// digits with no leading or trailing whitespace in addition to being the
/// right length and format.
pub fn parse(value: &str) -> Result<CIK, CIKError> {
    let s: String = value.into();

    if s.is_empty() || s.len() > 10 {
        Err(CIKError::InvalidLength { was: s.len() })
    } else {
        match s.parse::<u64>() {
            Ok(value) => build(value),
            Err(_err) => Err(CIKError::InvalidFormat { was: s }),
        }
    }
}

/// Build a CIK from an integer _Value_.
pub fn build(value: u64) -> Result<CIK, CIKError> {
    if !(1..=9_999_999_999).contains(&value) {
        return Err(CIKError::InvalidValue { was: value });
    }

    Ok(CIK(value))
}

/// Test whether or not the passed string is in valid CIK format, without producing a CIK struct
/// value.
pub fn validate(value: &str) -> bool {
    if value.is_empty() || value.len() > 10 {
        println!("Bad length: {:?}", value);
        return false;
    }

    // We make the preliminary assumption that the string is pure ASCII, so we work with the
    // underlying bytes. If there is Unicode in the string, the bytes will be outside the
    // allowed range and format validation will fail.

    let b = value.as_bytes();

    return b.iter().all(|b| *b >= b'0' && *b <= b'9');
}

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

/// A CIK in confirmed valid format.
///
/// You cannot construct a CIK value manually. This does not compile:
///
/// ```compile_fail
/// use cik;
/// let cannot_construct = cik::CIK(0);
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
#[repr(transparent)]
#[allow(clippy::upper_case_acronyms)]
pub struct CIK(u64);

impl fmt::Display for CIK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for CIK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CIK{}", self.0)
    }
}

impl FromStr for CIK {
    type Err = CIKError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl TryFrom<u64> for CIK {
    type Error = CIKError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        build(value)
    }
}

impl CIK {
    /// Return the underlying integer _Value_ of the CIK.
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_cik_for_apple() {
        match parse("320193") {
            Ok(cik) => {
                assert_eq!(cik.to_string(), "320193");
                assert_eq!(cik.value(), 320193);
            }
            Err(err) => assert!(false, "Did not expect parsing to fail: {}", err),
        }
    }
    #[test]
    fn build_cik_for_apple() {
        match build(320193) {
            Ok(cik) => {
                assert_eq!(cik.to_string(), "320193");
                assert_eq!(cik.value(), 320193);
            }
            Err(err) => assert!(false, "Did not expect building to fail: {}", err),
        }
    }

    #[test]
    fn reject_empty_string() {
        let res = parse("");
        assert!(res.is_err());
    }

    #[test]
    fn reject_non_digit_string() {
        let res = parse("-1");
        assert!(res.is_err());
    }

    #[test]
    fn reject_zero_string() {
        let res = parse("0");
        assert!(res.is_err());
    }

    #[test]
    fn reject_long_string() {
        let res = parse("10000000000");
        assert!(res.is_err());
    }

    #[test]
    fn reject_zero_value() {
        let res = build(0);
        assert!(res.is_err());
    }

    #[test]
    fn reject_large_value() {
        let res = build(10_000_000_000);
        assert!(res.is_err());
    }

    proptest! {
        #[test]
        #[allow(unused_must_use)]
        fn doesnt_crash(s in "\\PC*") {
            parse(&s);
        }
    }
}
