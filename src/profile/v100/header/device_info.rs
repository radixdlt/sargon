use chrono::NaiveDateTime;
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
    pub fn new(id: Uuid, date: NaiveDateTime, description: String) -> Self {
        Self {
            id,
            date,
            description,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::profile::v100::header::device_info::DeviceInfo;
    use chrono::NaiveDateTime;
    use core::fmt::Debug;
    use serde::{de::DeserializeOwned, ser::Serialize};
    use uuid::uuid;

    fn assert_json_roundtrip<T>(model: T, json_string: &str)
    where
        T: Serialize + DeserializeOwned + PartialEq + Debug,
    {
        let json = json_string.parse::<serde_json::Value>().unwrap();

        let deserialized: T = serde_json::from_value(json.clone()).unwrap();
        let serialized = serde_json::to_value(&model).unwrap();
        assert_eq!(deserialized, model);
        assert_eq!(serialized, json);
    }

    #[test]
    fn test_json_roundtrip() {
        assert_json_roundtrip(
            DeviceInfo::new(
                uuid!("66f07ca2-a9d9-49e5-8152-77aca3d1dd74"),
                NaiveDateTime::parse_from_str("2023-09-11T16:05:56", "%Y-%m-%dT%H:%M:%S").unwrap(),
                "iPhone (iPhone)".to_string(),
            ),
            r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56",
                "description": "iPhone (iPhone)"
            }
            "#,
        )
    }
}
