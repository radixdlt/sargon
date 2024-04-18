use crate::prelude::*;

/// A short summary of a device the Profile is being used
/// on, typically an iPhone or an Android phone.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{} | created: {} | #{}", description, self.date.date(), id.to_string())]
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
    pub date: Timestamp,

    /// A short description of the device, we devices should
    /// read the device model and a given name from the device
    /// if they are able to.
    ///
    /// E.g. "My Red Phone (iPhone SE 2nd Gen)"
    pub description: String,
}

impl DeviceInfo {
    /// Instantiates a new `DeviceInfo` with `id`, `date` and `description`.
    pub fn new(
        id: Uuid,
        date: Timestamp,
        description: impl AsRef<str>,
    ) -> Self {
        Self {
            id,
            date,
            description: description.as_ref().to_owned(),
        }
    }

    /// Instantiates a new `DeviceInfo` with `description`, and generates a new `id`
    /// and will use the current `date` for creation date.
    pub fn with_description(description: impl AsRef<str>) -> Self {
        Self::new(id(), now(), description)
    }

    /// Instantiates a new `DeviceInfo` with "iPhone" as description, and
    /// generates a new `id` and will use the current `date` for creation date.
    pub fn new_iphone() -> Self {
        Self::with_description("iPhone")
    }

    /// Instantiates a new `DeviceInfo` with "Unknown device" as description, and
    /// generates a new `id` and will use the current `date` for creation date.
    pub fn new_unknown_device() -> Self {
        Self::with_description("Unknown device")
    }
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self::new_unknown_device()
    }
}

impl HasSampleValues for DeviceInfo {
    fn sample() -> Self {
        Self::new(
            Uuid::from_str("66f07ca2-a9d9-49e5-8152-77aca3d1dd74").unwrap(),
            Timestamp::parse("2023-09-11T16:05:56.000Z").unwrap(),
            "iPhone",
        )
    }

    fn sample_other() -> Self {
        Self::new(
            Uuid::from_str("f07ca662-d9a9-9e45-1582-aca773d174dd").unwrap(),
            Timestamp::parse("2023-12-24T17:13:56.123Z").unwrap(),
            "Android",
        )
    }
}

impl DeviceInfo {
    /// Creates a new `DeviceInfo` from json in the form of `BagOfBytes`.
    /// This is a temporarily exported method that allows wallet clients to
    /// integrate Profile in steps.
    ///
    /// Should be replaced later with `Wallet`
    pub fn new_from_json_bytes(json: impl AsRef<str>) -> Result<Self> {
        let json = json.as_ref();
        serde_json::from_str::<Self>(json).map_err(|_| {
            CommonError::FailedToDeserializeJSONToValue {
                json_byte_count: json.len() as u64,
                type_name: "DeviceInfo".to_owned(),
            }
        })
    }

    /// Converts this `DeviceInfo` to json in the form of `BagOfBytes`
    /// This is a temporarily exported method that allows wallet clients to
    /// integrate Profile in steps.
    ///
    /// Should be replaced later with `Wallet`
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self)
            .expect("JSON serialization of DeviceInfo should never fail.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfo;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn new_iphone() {
        assert_eq!(SUT::new_iphone().description, "iPhone");
    }

    #[test]
    fn with_description() {
        assert_eq!(SUT::with_description("Nokia").description, "Nokia");
    }

    #[test]
    fn new_has_description_unknown_device() {
        assert_eq!(SUT::new_unknown_device().description, "Unknown device");
    }

    #[test]
    fn display() {
        let id_str = "12345678-bbbb-cccc-dddd-abcd12345678";
        let id = Uuid::from_str(id_str).unwrap();
        let sut = SUT::new(
            id,
            Timestamp::parse("2023-09-11T16:05:56Z").unwrap(),
            "Foo",
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
            .map(|_| SUT::new_iphone())
            .map(|d| d.id)
            .collect::<HashSet<Uuid>>();
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn date_is_now() {
        assert!(SUT::new_iphone().date.year() >= 2023);
    }

    #[test]
    fn json_string_roundtrip() {
        let sut = SUT::sample();
        let json_str = sut.to_json_string();
        assert_eq!(sut, SUT::new_from_json_bytes(json_str).unwrap());
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
        let model = serde_json::from_str::<SUT>(str).unwrap();
        assert_eq!(model.date.day(), 11);
        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("56.000Z"));
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
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
        assert_json_fails::<SUT>(
            r#"
            {
                "id": "invalid-uuid",
                "date": "1970-01-01T12:34:56.000Z",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "invalid-date",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "missing_key": "id",
                "date": "1970-01-01T12:34:56.000Z",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "missing_key": "date",
                "description": "iPhone"
            }
            "#,
        );

        assert_json_fails::<SUT>(
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
