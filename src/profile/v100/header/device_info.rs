use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// A best effort stable and unique identifier of this
    /// device.
    ///
    /// Apple has made it so that iOS devices cannot
    /// query iOS for a unique identifier of the device, thus
    /// the iOS team has made their own impl of a best effort
    /// stable identifier.
    pub id: Uuid,

    /// The date this description of the device was made, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub date: NaiveDateTime,

    /// A short description of the device, we devices should
    /// read the device model and a given name from the device
    /// if they are able to.
    ///
    /// E.g. "My Red Phone (iPhone SE 2nd Gen)"
    pub description: String,
}

impl DeviceInfo {
    pub fn with_values(id: Uuid, date: NaiveDateTime, description: String) -> Self {
        Self {
            id,
            date,
            description,
        }
    }

    pub fn new(description: &str) -> Self {
        Self::with_values(
            Uuid::new_v4(),
            Utc::now().naive_local(),
            description.to_string(),
        )
    }

    pub fn new_iphone() -> Self {
        Self::new("iPhone")
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::v100::header::device_info::DeviceInfo;
    use chrono::NaiveDateTime;
    use core::fmt::Debug;
    use serde::{de::DeserializeOwned, ser::Serialize};
    use serde_json::json;
    use std::any::TypeId;
    use uuid::uuid;

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
    fn assert_json_roundtrip<T>(model: &T)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let serialized = serde_json::to_value(&model).unwrap();
        let deserialized: T = serde_json::from_value(serialized.clone()).unwrap();
        assert_eq!(model, &deserialized);
    }

    fn assert_json_fails<T>(json_string: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json = json_string.parse::<serde_json::Value>().unwrap();
        let result = serde_json::from_value::<T>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_json() {
        let model = DeviceInfo::with_values(
            uuid!("66f07ca2-a9d9-49e5-8152-77aca3d1dd74"),
            NaiveDateTime::parse_from_str("2023-09-11T16:05:56", "%Y-%m-%dT%H:%M:%S").unwrap(),
            "iPhone".to_string(),
        );
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56",
                "description": "iPhone"
            }
            "#,
        );
        assert_json_roundtrip(&model);
        assert_ne_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "1970-01-01T12:34:56",
                "description": "Nokia"
            }
            "#,
        );
    }

    #[test]
    fn test_invalid_json() {
        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "id": "invalid-uuid",
                "date": "1970-01-01T12:34:56",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "invalid-date",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "missing_key": "id",
                "date": "1970-01-01T12:34:56",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "missing_key": "date",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "1970-01-01T12:34:56",
                "missing_key": "description"
            }
            "#,
        );
    }
}
