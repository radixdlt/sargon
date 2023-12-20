use std::fmt::Display;

use chrono::NaiveDateTime;
use derive_getters::Getters;
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wallet_kit_common::utils::factory::{date, id, now};

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, Getters)]
pub struct DeviceInfo {
    /// A best effort stable and unique identifier of this
    /// device.
    ///
    /// Apple has made it so that iOS devices cannot
    /// query iOS for a unique identifier of the device, thus
    /// the iOS team has made their own impl of a best effort
    /// stable identifier.
    id: Uuid,

    /// The date this description of the device was made, might
    /// be equal to when the app was first ever launched on the
    /// device.
    date: Timestamp,

    /// A short description of the device, we devices should
    /// read the device model and a given name from the device
    /// if they are able to.
    ///
    /// E.g. "My Red Phone (iPhone SE 2nd Gen)"
    description: String,
}

impl DeviceInfo {
    /// Instantiates a new `DeviceInfo` with `id`, `date` and `description`.
    pub fn with_values(id: Uuid, date: Timestamp, description: String) -> Self {
        Self {
            id,
            date,
            description,
        }
    }

    /// Instantiates a new `DeviceInfo` with `description`, and generates a new `id`
    /// and will use the current `date` for creation date.
    pub fn with_description(description: &str) -> Self {
        Self::with_values(id(), Timestamp::now_utc(), description.to_string())
    }

    /// Instantiates a new `DeviceInfo` with "iPhone" as description, and
    /// generates a new `id` and will use the current `date` for creation date.
    pub fn new_iphone() -> Self {
        Self::with_description("iPhone")
    }

    /// Instantiates a new `DeviceInfo` with "Unknown device" as description, and
    /// generates a new `id` and will use the current `date` for creation date.
    pub fn new() -> Self {
        Self::with_description("Unknown device")
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | created: {} | #{}",
            self.description,
            &self.date.to_string(),
            self.id.to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, str::FromStr};

    use crate::v100::header::device_info::DeviceInfo;
    use iso8601_timestamp::Timestamp;
    use uuid::Uuid;
    use wallet_kit_common::json::*;

    #[test]
    fn new_iphone() {
        assert_eq!(DeviceInfo::new_iphone().description, "iPhone");
    }

    #[test]
    fn with_description() {
        assert_eq!(DeviceInfo::with_description("Nokia").description, "Nokia");
    }

    #[test]
    fn new_has_description_unknown_device() {
        assert_eq!(DeviceInfo::new().description, "Unknown device");
    }

    #[test]
    fn display() {
        let id_str = "12345678-bbbb-cccc-dddd-abcd12345678";
        let id = Uuid::from_str(id_str).unwrap();
        let sut = DeviceInfo::with_values(
            id,
            Timestamp::parse("2023-09-11T16:05:56Z").unwrap(),
            "Foo".to_string(),
        );
        assert_eq!(
            format!("{sut}"),
            format!("Foo | created: 2023-09-11 | #{}", id_str)
        )
    }

    #[test]
    fn id_is_unique() {
        let n = 1000;
        let ids = (0..n)
            .map(|_| DeviceInfo::new_iphone())
            .into_iter()
            .map(|d| d.id)
            .collect::<HashSet<Uuid>>();
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn date_is_now() {
        assert!(DeviceInfo::new_iphone().date.year() >= 2023);
    }

    #[test]
    fn can_parse_iso8601_json_without_milliseconds_precision() {
        let str = r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56Z",
                "description": "iPhone"
            }
            "#;
        let model = serde_json::from_str::<DeviceInfo>(str).unwrap();
        assert_eq!(model.date().day(), 11);
        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("56.000Z")); // but when serialized, `.000` is included.
    }

    #[test]
    fn json_roundtrip() {
        let model = DeviceInfo::with_values(
            Uuid::from_str("66f07ca2-a9d9-49e5-8152-77aca3d1dd74").unwrap(),
            Timestamp::parse("2023-09-11T16:05:56.000Z").unwrap(),
            "iPhone".to_string(),
        );
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56.000Z",
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
                "date": "1970-01-01T12:34:56.000Z",
                "description": "Nokia"
            }
            "#,
        );
    }

    #[test]
    fn invalid_json() {
        assert_json_fails::<DeviceInfo>(
            r#"
            {
                "id": "invalid-uuid",
                "date": "1970-01-01T12:34:56.000Z",
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
                "date": "1970-01-01T12:34:56.000Z",
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
                "date": "1970-01-01T12:34:56.000Z",
                "missing_key": "description"
            }
            "#,
        );
    }
}
