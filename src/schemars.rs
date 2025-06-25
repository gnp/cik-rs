//! # cik::schemars
//!
//! JSON Schema support for CIKs.
//!
//! This module provides [schemars](https://crates.io/crates/schemars) support for generating
//! JSON schemas that describe the CIK data format. The generated schema accepts both
//! integer values (0 to 9,999,999,999) and string values (1 to 10 digits), matching
//! the behavior of the serde deserialization support.
//!
//! ## Example
//!
//! ```rust
//! # #[cfg(feature = "schemars")] {
//! use cik::CIK;
//! use schemars::schema_for;
//!
//! let schema = schema_for!(CIK);
//! let schema_json = serde_json::to_string_pretty(&schema).unwrap();
//! println!("{}", schema_json);
//! # }
//! ```

use self::cik::CIK;
use crate as cik;
use schemars::JsonSchema;

impl JsonSchema for CIK {
    fn schema_name() -> String {
        "CIK".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{
            InstanceType, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
        };

        // Integer schema (0 to 9,999,999,999)
        let integer_schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::Integer))),
            number: Some(Box::new(schemars::schema::NumberValidation {
                minimum: Some(0.0),
                maximum: Some(9_999_999_999.0),
                ..Default::default()
            })),
            ..Default::default()
        };

        // String schema (digits only, 1-10 chars)
        let string_schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            string: Some(Box::new(schemars::schema::StringValidation {
                pattern: Some(r"^\d{1,10}$".to_string()),
                ..Default::default()
            })),
            ..Default::default()
        };

        // Combined schema that accepts either integer or string
        let schema = SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                any_of: Some(vec![
                    Schema::Object(integer_schema),
                    Schema::Object(string_schema),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        };

        schema.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use serde_json::json;

    #[test]
    fn schema_generation() {
        let schema = ::schemars::schema_for!(CIK);
        let schema_json = serde_json::to_value(&schema).unwrap();

        // Verify the schema has the expected structure
        assert_eq!(schema_json["title"], "CIK");
        assert!(schema_json["anyOf"].is_array());
        assert_eq!(schema_json["anyOf"][0]["type"], "integer");
        assert_eq!(schema_json["anyOf"][1]["type"], "string");
    }

    #[test]
    fn schema_validation() {
        let schema = ::schemars::schema_for!(CIK);
        let schema_json = serde_json::to_value(&schema).unwrap();
        let compiled = jsonschema::Validator::new(&schema_json).unwrap();

        // Valid CIK values should pass (both integers and strings)
        assert!(compiled.is_valid(&json!(320193)));
        assert!(compiled.is_valid(&json!("320193")));
        assert!(compiled.is_valid(&json!(1)));
        assert!(compiled.is_valid(&json!("1")));
        assert!(compiled.is_valid(&json!(9_999_999_999u64)));
        assert!(compiled.is_valid(&json!("9999999999")));

        // Edge case: leading zeros (schema allows, but CIK parsing should handle)
        assert!(compiled.is_valid(&json!("0000000001"))); // Schema allows this

        // Invalid values should fail
        assert!(!compiled.is_valid(&json!(-1)));
        assert!(!compiled.is_valid(&json!(10_000_000_000u64)));
        assert!(!compiled.is_valid(&json!(""))); // Empty string
        assert!(!compiled.is_valid(&json!("12345678901"))); // Too long
        assert!(!compiled.is_valid(&json!("123ABC"))); // Non-digits
        assert!(!compiled.is_valid(&json!(3.5))); // Float
    }

    #[test]
    #[cfg(feature = "serde")]
    fn schema_matches_serde() {
        // When both features are enabled, verify that the schema matches serde behavior
        let cik = build(320193).unwrap();

        // Serialize with serde
        let serialized = serde_json::to_value(cik).unwrap();

        // Get the schema
        let schema = ::schemars::schema_for!(CIK);
        let schema_json = serde_json::to_value(&schema).unwrap();
        let compiled = jsonschema::Validator::new(&schema_json).unwrap();

        // The serialized value should validate against the schema
        assert!(compiled.is_valid(&serialized));
    }

    #[test]
    #[cfg(all(feature = "serde", feature = "schemars"))]
    fn struct_with_cik_roundtrip() {
        // Example JSON with Apple's CIK
        let json_str = r#"{
            "name": "Apple Inc.",
            "cik": 320193,
            "active": true
        }"#;

        // Parse the JSON as a serde_json::Value for schema validation
        let json_value: serde_json::Value = serde_json::from_str(json_str).unwrap();

        // Generate schema for CIK to show it validates integer values
        let cik_schema = ::schemars::schema_for!(CIK);
        let cik_schema_json = serde_json::to_value(&cik_schema).unwrap();
        let compiled = jsonschema::Validator::new(&cik_schema_json).unwrap();

        // Verify the CIK value from JSON passes schema validation
        assert!(compiled.is_valid(&json_value["cik"]));

        // Test direct CIK serialization/deserialization
        let test_cik = build(320193).unwrap();

        // Serialize CIK with serde
        let serialized_cik = serde_json::to_value(test_cik).unwrap();

        // Verify the serialized CIK passes schema validation
        assert!(compiled.is_valid(&serialized_cik));

        // Verify the serialized value matches what we expect
        assert_eq!(serialized_cik, 320193);

        // Test deserialization of the CIK value from the original JSON
        let deserialized_cik: CIK = serde_json::from_value(json_value["cik"].clone()).unwrap();
        assert_eq!(deserialized_cik.value(), 320193);

        // Test edge case: leading zeros (should work with serde)
        let leading_zero_cik: CIK = serde_json::from_str("\"0000000001\"").unwrap();
        assert_eq!(leading_zero_cik.value(), 1);

        // Re-serialize and verify it still passes validation
        let reserialized_cik = serde_json::to_value(deserialized_cik).unwrap();
        assert!(compiled.is_valid(&reserialized_cik));
        assert_eq!(reserialized_cik, 320193);
    }
}
