use crate::prelude::*;

/// A name and model of a host device.
///
/// This used to be a String only in Pre 1.6.0 wallets, so
/// we have a custom Deserialize impl of it.
#[derive(
    Serialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    uniffi::Record,
)]
#[display("{name} ({model})")]
pub struct DeviceInfoDescription {
    /// Host device name, e.g. "My Precious"
    pub name: String,

    /// Host device model, e.g. "iPhone 15 Pro"
    pub model: String,
}

impl DeviceInfoDescription {
    pub fn new(name: impl AsRef<str>, model: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            model: model.as_ref().to_owned(),
        }
    }
}

impl<'de> Deserialize<'de> for DeviceInfoDescription {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct NewFormat {
            pub name: String,
            pub model: String,
        }

        #[derive(Deserialize, Serialize)]
        #[serde(untagged)]
        enum Wrapper {
            NewFormat(NewFormat),
            OldFormat(String),
        }

        match Wrapper::deserialize(deserializer)? {
            Wrapper::NewFormat(new_format) => Ok(Self {
                name: new_format.name,
                model: new_format.model,
            }),
            Wrapper::OldFormat(description) => {
                Ok(Self::from(description.as_ref()))
            }
        }
    }
}

impl From<&str> for DeviceInfoDescription {
    fn from(description: &str) -> Self {
        // Swift used: "\(model) (\(name))"
        let swift_name_suffix = " (iPhone)";
        if description.ends_with(swift_name_suffix) {
            let model = description.split(swift_name_suffix).next().unwrap();
            return Self::new("iPhone", model);
        }
        // FIXME: Android
        Self::new(description, description)
    }
}

impl HasSampleValues for DeviceInfoDescription {
    fn sample() -> Self {
        Self::new("My precious", "iPhone 15 Pro")
    }

    fn sample_other() -> Self {
        Self::new("R2", "OnePlus Open")
    }
}

#[cfg(test)]
mod test_device_info_description {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceInfoDescription;

    #[test]
    fn json_new_format() {
        let sut = SUT::sample();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "My precious",
                "model": "iPhone 15 Pro"
            }
            "#,
        )
    }

    #[test]
    fn json_old_format_iphone_iphone() {
        let json = json!("iPhone (iPhone)");
        let sut = serde_json::from_value::<SUT>(json).unwrap();
        assert_eq!(sut.clone(), SUT::new("iPhone", "iPhone"));

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "iPhone",
                "model": "iPhone"
            }
            "#,
        );
    }

    #[test]
    fn json_old_format_iphone15_pro_max_iphone() {
        let json = json!("iPhone 15 Pro Max (iPhone)");
        let sut = serde_json::from_value::<SUT>(json).unwrap();
        assert_eq!(sut.clone(), SUT::new("iPhone", "iPhone 15 Pro Max"));

        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "iPhone",
                "model": "iPhone 15 Pro Max"
            }
            "#,
        );
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
