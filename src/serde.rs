//! # cik::serde
//!
//! Serde support for CIKs.

use self::cik::CIK;
use crate as cik;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

impl Serialize for cik::CIK {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

struct CIKVisitor;
impl<'de> Visitor<'de> for CIKVisitor {
    type Value = CIK;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a positive integer up to 10 digits or a string representing the same")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match cik::build(value) {
            Ok(cik) => Ok(cik),
            Err(err) => Err(E::custom(format!("Cannot deserialize {}: {}", value, err))),
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value < 0 {
            Err(E::custom(format!(
                "Cannot deserialize {}: Negative values not allowed",
                value
            )))
        } else {
            self.visit_u64(u64::try_from(value).unwrap())
        }
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match cik::parse(s) {
            Ok(cik) => Ok(cik),
            Err(err) => Err(E::custom(format!("Cannot deserialize {}: {}", s, err))),
        }
    }
}

impl<'de> Deserialize<'de> for CIK {
    fn deserialize<D>(deserializer: D) -> Result<CIK, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CIKVisitor)
        // deserializer.deserialize_u64(CIKVisitor).or_else(|_| deserializer.deserialize_str(CIKVisitor))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    // use crate::CIK;
    use serde_json;

    #[test]
    fn serialize() {
        let test_cik: CIK = build(320193).unwrap(); // Apple

        let serialized = serde_json::to_string(&test_cik).unwrap();
        println!("serialized = {}", serialized);
        assert_eq!(serialized.to_string(), "320193");
    }

    #[test]
    fn deserialize_u64() {
        let json = "320193";
        let test_cik: CIK = build(320193).unwrap(); // Apple

        let deserialized: CIK = serde_json::from_str(json).unwrap();

        println!("deserialized = {}", deserialized);
        assert_eq!(deserialized, test_cik);
    }

    #[test]
    fn deserialize_string() {
        let json = "\"320193\"";
        let test_cik: CIK = build(320193).unwrap(); // Apple

        let deserialized: CIK = serde_json::from_str(json).unwrap();

        println!("deserialized = {}", deserialized);
        assert_eq!(deserialized, test_cik);
    }
}
