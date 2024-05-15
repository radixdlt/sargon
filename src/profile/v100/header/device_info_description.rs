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
                let name = description.clone();
                let model = description.clone();

                // let re = Regex::new(r"(?<name>[alphanum]+)((?<model>\d{4})\)").unwrap();

                // let parts = description.split("(").collect_vec();
                // if parts.len() >= 2 {
                //     name = parts[0].to_owned();
                //     if let Some(model_to_be_parsed) =
                //         description.strip_prefix(&name)
                //     {
                //         if model_to_be_parsed.starts_with("(")
                //             && model_to_be_parsed.ends_with(")")
                //         {
                //             // iOS styled
                //             model = model_to_be_parsed
                //                 [1..model_to_be_parsed.len() - 1]
                //                 .to_owned();
                //         } else {
                //             model = model_to_be_parsed.to_owned();
                //         }
                //     }
                // }
                Ok(Self { name, model })
            }
        }
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
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
