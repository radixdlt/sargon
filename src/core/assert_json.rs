use assert_json_diff::assert_json_include;
use core::fmt::Debug;
use pretty_assertions::{assert_eq, assert_ne};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::Value;

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
        assert_eq!(&deserialized, model, "Expected `model: T` and `T` deserialized from `json_string`, to be equal, but they were not.");
        assert_json_include!(actual: serialized, expected: json);
    } else {
        assert_ne!(model, &deserialized);
        assert_ne!(&deserialized, model, "Expected difference between `model: T` and `T` deserialized from `json_string`, but they were unexpectedly equal.");
        assert_ne!(serialized, json, "Expected difference between `json` (string) and json serialized from `model`, but they were unexpectedly equal.");
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
