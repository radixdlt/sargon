#[cfg(test)]
pub mod json_test_utils {
    use core::fmt::Debug;
    use serde::{de::DeserializeOwned, ser::Serialize};

    fn base_assert_equality_after_json_roundtrip<T>(model: &T, json_string: &str, expect_eq: bool)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let serialized = serde_json::to_value(&model).unwrap();
        let json = json_string.parse::<serde_json::Value>().unwrap();
        let deserialized: T = serde_json::from_value(json.clone()).unwrap();
        if expect_eq {
            assert_eq!(&deserialized, model, "Expected `model: T` and `T` deserialized from `json_string`, to be equal, but they were not.");
            assert_eq!(serialized, json, "Expected `json` (string) and json serialized from `model to be equal`, but they were not.");
        } else {
            assert_ne!(&deserialized, model, "Expected difference between `model: T` and `T` deserialized from `json_string`, but they were unexpectedly equal.");
            assert_ne!(serialized, json, "Expected difference between `json` (string) and json serialized from `model`, but they were unexpectedly equal.");
        }
    }

    /// Asserts that (pseudocode) `model.to_json() == json_string` (serialization)
    /// and also asserts the associative property:
    /// `Model::from_json(json_string) == model` (deserialization)
    pub fn assert_eq_after_json_roundtrip<T>(model: &T, json_string: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        base_assert_equality_after_json_roundtrip(model, json_string, true)
    }

    /// Asserts that (pseudocode) `model.to_json() != json_string` (serialization)
    /// and also asserts the associative property:
    /// `Model::from_json(json_string) != model` (deserialization)
    pub fn assert_ne_after_json_roundtrip<T>(model: &T, json_string: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        base_assert_equality_after_json_roundtrip(model, json_string, false)
    }

    /// Asserts that (pseudocode) `Model::from_json(model.to_json()) == model`,
    /// i.e. that a model after JSON roundtripping remain unchanged.
    pub fn assert_json_roundtrip<T>(model: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let serialized = serde_json::to_value(&model).unwrap();
        let deserialized: T = serde_json::from_value(serialized.clone()).unwrap();
        assert_eq!(model, &deserialized);
    }

    pub fn assert_json_fails<T>(json_string: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json = json_string.parse::<serde_json::Value>().unwrap();
        let result = serde_json::from_value::<T>(json);
        assert!(result.is_err());
    }
}
