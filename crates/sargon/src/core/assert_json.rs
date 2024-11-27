use crate::prelude::*;

use assert_json_diff::assert_json_include;
use core::fmt::Debug;
use pretty_assertions::{assert_eq, assert_ne};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum TestingError {
    #[error("File contents is not valid JSON '{0}'")]
    FailedDoesNotContainValidJSON(String),

    #[error("Failed to JSON deserialize string")]
    FailedToDeserialize(serde_json::Error),
}

/// `name` is file name without extension, assuming it is json file
#[cfg(not(tarpaulin_include))]
pub fn fixture_and_json<'a, T>(
    vector: &str,
) -> Result<(T, serde_json::Value), TestingError>
where
    T: for<'de> Deserialize<'de>,
{
    let json = serde_json::Value::from_str(vector).map_err(|_| {
        TestingError::FailedDoesNotContainValidJSON(vector.to_owned())
    })?;

    serde_json::from_value::<T>(json.clone())
        .map_err(TestingError::FailedToDeserialize)
        .map(|v| (v, json))
}

/// `name` is file name without extension, assuming it is json file
#[cfg(not(tarpaulin_include))]
#[allow(unused)]
pub fn fixture<'a, T>(vector: &str) -> Result<T, TestingError>
where
    T: for<'de> Deserialize<'de>,
{
    fixture_and_json(vector).map(|t| t.0)
}

#[cfg(not(tarpaulin_include))]
fn base_assert_equality_after_json_roundtrip<T>(
    model: &T,
    json: Value,
    expect_eq: bool,
) where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let serialized = serde_json::to_value(model).unwrap();
    let deserialized: T = serde_json::from_value(json.clone()).unwrap();
    if expect_eq {
        pretty_assertions::assert_eq!(&deserialized, model, "Expected `model: T` and `T` deserialized from `json_string`, to be equal, but they were not.");
        assert_json_include!(actual: serialized, expected: json);
    } else {
        pretty_assertions::assert_ne!(model, &deserialized);
        pretty_assertions::assert_ne!(&deserialized, model, "Expected difference between `model: T` and `T` deserialized from `json_string`, but they were unexpectedly equal.");
        pretty_assertions::assert_ne!(serialized, json, "Expected difference between `json` (string) and json serialized from `model`, but they were unexpectedly equal.");
    }
}

/// Asserts that (pseudocode) `model.to_json() == json_string` (serialization)
/// and also asserts the associative property:
/// `Model::from_json(json_string) == model` (deserialization)
#[cfg(not(tarpaulin_include))]
pub fn assert_eq_after_json_roundtrip<T>(model: &T, json_string: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = json_string.parse::<serde_json::Value>().unwrap();
    base_assert_equality_after_json_roundtrip(model, json, true)
}

#[cfg(not(tarpaulin_include))]
pub fn print_json<T>(model: &T)
where
    T: Serialize,
{
    println!(
        "{}",
        serde_json::to_string_pretty(model).expect(
            "Should be able to JSON serialize passed in serializable model."
        )
    );
}

/// Asserts that (pseudocode) `model.to_json() == json` (serialization)
/// and also asserts the associative property:
/// `Model::from_json(json) == model` (deserialization)
#[cfg(not(tarpaulin_include))]
pub fn assert_json_value_eq_after_roundtrip<T>(model: &T, json: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    base_assert_equality_after_json_roundtrip(model, json, true)
}

/// Asserts that (pseudocode) `model.to_json() != json_string` (serialization)
/// and also asserts the associative property:
/// `Model::from_json(json_string) != model` (deserialization)
#[cfg(not(tarpaulin_include))]
pub fn assert_ne_after_json_roundtrip<T>(model: &T, json_string: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = json_string.parse::<serde_json::Value>().unwrap();
    base_assert_equality_after_json_roundtrip(model, json, false)
}

/// Asserts that (pseudocode) `model.to_json() != json` (serialization)
/// and also asserts the associative property:
/// `Model::from_json(json) != model` (deserialization)
#[cfg(not(tarpaulin_include))]
pub fn assert_json_value_ne_after_roundtrip<T>(model: &T, json: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    base_assert_equality_after_json_roundtrip(model, json, false)
}

/// Asserts that (pseudocode) `Model::from_json(model.to_json()) == model`,
/// i.e. that a model after JSON roundtripping remain unchanged.
#[cfg(not(tarpaulin_include))]
pub fn assert_json_roundtrip<T>(model: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let serialized = serde_json::to_value(model).unwrap();
    let deserialized: T = serde_json::from_value(serialized.clone()).unwrap();
    assert_eq!(model, &deserialized);
}

/// Creates JSON from `json_str` and tries to decode it, then encode the decoded,
/// value and compare it to the JSON value of the json_str.
#[cfg(not(tarpaulin_include))]
pub fn assert_json_str_roundtrip<T>(json_str: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let value = serde_json::Value::from_str(json_str).unwrap();
    let deserialized: T = serde_json::from_value(value.clone()).unwrap();
    let serialized = serde_json::to_value(&deserialized).unwrap();
    assert_eq!(value, serialized);
}

#[cfg(not(tarpaulin_include))]
pub fn assert_json_value_fails<T>(json: Value)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let result = serde_json::from_value::<T>(json.clone());

    if let Ok(t) = result {
        panic!(
            "Expected JSON serialization to fail, but it did not, deserialized into: {:?},\n\nFrom JSON: {}",
            t,
            serde_json::to_string(&json).unwrap()
        );
    }
    // all good, expected fail.
}

#[cfg(not(tarpaulin_include))]
pub fn assert_json_fails<T>(json_string: &str)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = json_string.parse::<serde_json::Value>().unwrap();
    assert_json_value_fails::<T>(json)
}

#[cfg(not(tarpaulin_include))]
pub fn assert_json_eq_ignore_whitespace(json1: &str, json2: &str) {
    let value1: Value =
        serde_json::from_str(json1).expect("Invalid JSON in json1");
    let value2: Value =
        serde_json::from_str(json2).expect("Invalid JSON in json2");
    assert_eq!(value1, value2, "JSON strings do not match");
}
