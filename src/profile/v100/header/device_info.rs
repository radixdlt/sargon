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
    pub id: DeviceID,

    /// The date this description of the device was made, might
    /// be equal to when the app was first ever launched on the
    /// device.
    pub date: Timestamp,

    /// A short description of the device, we devices should
    /// read the device model and a given name from the device
    /// if they are able to.
    pub description: String, // FIXME: Start using `DeviceInfoDescription` !

    /// The **last known** version of the device's operating system, e.g. "iOS 17.4.1".
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub system_version: Option<String>,

    /// The **last known** version of the host app, for example the Radix iOS Wallet version - e.g. "1.6.1"
    ///
    /// It is possible that the host device has been updated to a new
    /// version than recorded here, but Sargon or host clients might
    /// just not have updated this value here.
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_app_version: Option<String>,

    /// The vendor of the host client, e.g. "Apple" for iPhone clients,
    /// or "Samsung" for Android clients.
    ///
    /// MUST be optional since this was added on 2024-05-16 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_vendor: Option<String>,
}

impl DeviceInfo {
    /// Instantiates a new `DeviceInfo` with `id`, `date` and `description`.
    pub fn new(
        id: DeviceID,
        date: Timestamp,
        description: DeviceInfoDescription,
        system_version: impl AsRef<str>,
        host_app_version: impl AsRef<str>,
        host_vendor: impl AsRef<str>,
    ) -> Self {
        Self {
            id,
            date,
            description: description.to_string(),
            system_version: Some(system_version.as_ref().to_owned()),
            host_app_version: Some(host_app_version.as_ref().to_owned()),
            host_vendor: Some(host_vendor.as_ref().to_owned()),
        }
    }

    /// Instantiates a new `DeviceInfo` with all needed details,
    /// formatting a `description` from host name and host model.
    pub fn with_details(
        name: impl AsRef<str>,
        model: impl AsRef<str>,
        system_version: impl AsRef<str>,
        host_app_version: impl AsRef<str>,
        host_vendor: impl AsRef<str>,
    ) -> Self {
        Self::new(
            DeviceID::generate_new(),
            now(),
            DeviceInfoDescription::new(name, model),
            system_version,
            host_app_version,
            host_vendor,
        )
    }
}

#[cfg(test)]
impl DeviceInfo {
    pub fn new_unknown() -> Self {
        Self::with_details(
            "Unknown", "Unknown", "Unknown", "Unknown", "Unknown",
        )
    }
}

impl HasSampleValues for DeviceInfo {
    fn sample() -> Self {
        Self {
            id: DeviceID::from_str("66F07CA2-A9D9-49E5-8152-77ACA3D1DD74")
                .unwrap(),
            date: Timestamp::sample(),
            description: DeviceInfoDescription {
                name: "iPhone".to_owned(),
                model: "iPhone".to_owned(),
            }
            .to_string(),
            system_version: None,
            host_app_version: None,
            host_vendor: None,
        }
    }

    fn sample_other() -> Self {
        Self {
            id: DeviceID::from_str("f07ca662-d9a9-9e45-1582-aca773d174dd")
                .unwrap(),
            date: Timestamp::sample_other(),
            description: DeviceInfoDescription {
                name: "Android".to_owned(),
                model: "Android".to_owned(),
            }
            .to_string(),
            system_version: None,
            host_app_version: None,
            host_vendor: None,
        }
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
    fn with_details() {
        assert_eq!(
            SUT::with_details(
                "My precious",
                "iPhone SE 2nd gen",
                "iOS 17.4.1",
                "1.6.4",
                "Apple"
            )
            .description
            .to_string(),
            "My precious (iPhone SE 2nd gen)"
        );
    }

    #[test]
    fn display() {
        let id_str = "12345678-bbbb-cccc-dddd-abcd12345678";
        let id = DeviceID::from_str(id_str).unwrap();
        let sut = SUT::new(
            id,
            Timestamp::parse("2023-09-11T16:05:56Z").unwrap(),
            DeviceInfoDescription::new("Foo", "Unknown"),
            "Unknown",
            "Unknown",
            "Unknown",
        );
        pretty_assertions::assert_eq!(
            format!("{sut}"),
            format!("Foo (Unknown) | created: 2023-09-11 | #{}", id_str)
        )
    }

    #[test]
    fn id_is_unique() {
        let n = 20;
        let ids = (0..n)
            .map(|_| SUT::new_unknown())
            .map(|d| d.id)
            .collect::<HashSet<DeviceID>>();
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn date_is_now() {
        assert!(SUT::new_unknown().date.year() >= 2023);
    }

    #[test]
    fn can_parse_iso8601_json_without_milliseconds_precision() {
        let str = r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56Z",
                "description": "iPhone (iPhone)"
            }
            "#;
        let model = serde_json::from_str::<SUT>(str).unwrap();
        assert_eq!(model.date.day(), 11);
        let json = serde_json::to_string(&model).unwrap();
        assert!(json.contains("56.000Z"));
    }

    #[test]
    fn json_nanoseconds_precision() {
        assert_json_roundtrip(&SUT::new_unknown());
    }

    #[test]
    fn json_roundtrip() {
        let model = SUT::sample();
        assert_eq_after_json_roundtrip(
            &model,
            // The JSON string literal below contains `.000` ISO8601
            // milliseconds which is not set on the sample
            r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56.000Z",
                "description": "iPhone (iPhone)"
            }
            "#,
        );
        assert_json_roundtrip(&model);
        assert_ne_after_json_roundtrip(
            &model,
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "1970-01-01T12:34:56Z",
                "description": "Nokia 3310 (lur)"
            }
            "#,
        );
    }

    #[test]
    fn json_roundtrip_with_system_and_app_version() {
        let sut = SUT::new(
            DeviceID::from_str("66F07CA2-A9D9-49E5-8152-77ACA3D1DD74").unwrap(),
            Timestamp::parse("2023-09-11T16:05:56Z").unwrap(),
            DeviceInfoDescription::new("My nice iPhone", "iPhone 15 Pro"),
            "17.4.1",
            "1.6.0",
            "Apple",
        );
        assert_eq_after_json_roundtrip(
            &sut,
            // The JSON string literal below contains `.000` ISO8601
            // milliseconds which is not set on the sample
            r#"
            {
                "id": "66f07ca2-a9d9-49e5-8152-77aca3d1dd74",
                "date": "2023-09-11T16:05:56.000Z",
                "description": "My nice iPhone (iPhone 15 Pro)", 
                "system_version": "17.4.1",
                "host_app_version": "1.6.0",
                "host_vendor": "Apple"
            }
            "#,
        )
    }

    #[test]
    fn invalid_json() {
        assert_json_fails::<SUT>(
            r#"
            {
                "id": "invalid-uuid",
                "date": "1970-01-01T12:34:56.000Z",
                "description": "iPhone (iPhone)"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "date": "invalid-date",
                "description": "iPhone (iPhone)"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "missing_key": "id",
                "date": "1970-01-01T12:34:56.000Z",
                "description": "iPhone (iPhone)"
            }
            "#,
        );

        assert_json_fails::<SUT>(
            r#"
            {
                "id": "00000000-0000-0000-0000-000000000000",
                "missing_key": "date",
                "description": "iPhone (iPhone)"
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
