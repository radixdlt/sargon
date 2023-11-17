use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::factory::{id, now};

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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
    /// Instantiates a new `DeviceInfo` with `id`, `date` and `description`.
    pub fn with_values(id: Uuid, date: NaiveDateTime, description: String) -> Self {
        Self {
            id,
            date,
            description,
        }
    }

    /// Instantiates a new `DeviceInfo` with `description`, and generates a new `id`
    /// and will use the current `date` for creation date.
    pub fn with_description(description: &str) -> Self {
        Self::with_values(id(), now(), description.to_string())
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::v100::header::device_info::DeviceInfo;
    use chrono::{Datelike, NaiveDateTime};
    use uuid::{uuid, Uuid};
    use wallet_kit_test_utils::json::*;

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
    fn json_roundtrip() {
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
    fn invalid_json() {
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
