#![warn(missing_docs)]
//! # cik::error
//!
//! Error type for CIK parsing and building.

use std::error::Error;
use std::fmt::Formatter;
use std::fmt::{Debug, Display};

/// All the ways parsing or building could fail.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq)]
pub enum CIKError {
    /// The input length is not 1 to 10 bytes.
    InvalidLength {
        /// The length we found
        was: usize,
    },
    /// The input does not parse as an integer.
    InvalidFormat {
        /// The input string
        was: String,
    },
    /// The value is not a positive number of up to 10 digits (checked when building).
    InvalidValue {
        /// The length we found
        was: u64,
    },
}

impl Debug for CIKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CIKError::InvalidLength { was } => {
                write!(f, "InvalidLength {{ was: {:?} }}", was)
            }
            CIKError::InvalidFormat { was } => {
                write!(f, "InvalidFormat {{ was: {:?} }}", was)
            }
            CIKError::InvalidValue { was } => {
                write!(f, "InvalidValue {{ was: {:?} }}", was)
            }
        }
    }
}

impl Display for CIKError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CIKError::InvalidLength { was } => {
                write!(f, "invalid length {} bytes when expecting 1 to 10", was)
            }
            CIKError::InvalidFormat { was } => {
                write!(f, "invalid format {} when expecting integer", was)
            }
            CIKError::InvalidValue { was } => {
                write!(
                    f,
                    "invalid value {} when expecting positive number up to 9,999,999,999",
                    was
                )
            }
        }
    }
}

impl Error for CIKError {}
