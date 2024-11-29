use crate::prelude::*;

/// Properties describing a DeviceFactorSource to help user disambiguate between
/// it and another one.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{label} {device_name} {model}")]
pub struct DeviceFactorSourceHint {
    ///
    #[serde(default = "label_default")]
    pub label: DisplayName,

    /// "iPhone RED"
    #[serde(rename = "name")]
    pub device_name: String,

    /// "iPhone SE 2nd gen"
    pub model: String,

    /// The number of words in the mnemonic of a DeviceFactorSource, according to the BIP39
    /// standard, a multiple of 3, from 12 to 24 words.
    pub mnemonic_word_count: BIP39WordCount,

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

    /// The vendor of the device host, e.g. "Apple" or "Samsung".
    ///
    /// MUST be optional since this was added on 2024-05-03 and
    /// was not present in earlier version of wallet (pre 1.6.0).
    pub host_vendor: Option<String>,
}

impl DeviceFactorSourceHint {
    /// Instantiates a new DeviceFactorSourceHint from the specified name, model,
    ///  system version, app version and mnemonic word count.
    pub fn new(
        label: DisplayName,
        device_name: impl AsRef<str>,
        model: impl AsRef<str>,
        system_version: impl Into<Option<String>>,
        host_app_version: impl Into<Option<String>>,
        host_vendor: impl Into<Option<String>>,
        word_count: BIP39WordCount,
    ) -> Self {
        Self {
            label,
            device_name: device_name.as_ref().to_owned(),
            model: model.as_ref().to_owned(),
            system_version: system_version.into(),
            host_app_version: host_app_version.into(),
            host_vendor: host_vendor.into(),
            mnemonic_word_count: word_count,
        }
    }

    pub fn with_info(host_info: &HostInfo, word_count: BIP39WordCount) -> Self {
        let description = host_info.description.clone();
        Self::new(
            label_default(),
            description.name,
            description.model,
            host_info.host_os.version(),
            host_info.host_app_version.clone(),
            host_info.host_os.vendor(),
            word_count,
        )
    }
}

impl HasSampleValues for DeviceFactorSourceHint {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::new(
            DisplayName::new("New Phone").unwrap(),
            "Unknown Device Name",
            "iPhone",
            None,
            None,
            None,
            BIP39WordCount::TwentyFour,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            DisplayName::new("Old Phone").unwrap(),
            "Android",
            "Samsung Galaxy S23 Ultra",
            None,
            None,
            None,
            BIP39WordCount::Twelve,
        )
    }
}

impl DeviceFactorSourceHint {
    /// A sample used to facilitate unit tests.
    pub fn sample_iphone_unknown() -> Self {
        Self::new(
            DisplayName::new("Unknown Phone").unwrap(),
            "Unknown Device Name",
            "iPhone",
            None,
            None,
            None,
            BIP39WordCount::TwentyFour,
        )
    }
}

fn label_default() -> DisplayName {
    DisplayName::new("My Phone").unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DeviceFactorSourceHint;

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
    fn get_word_count() {
        assert_eq!(
            SUT::sample().mnemonic_word_count,
            BIP39WordCount::TwentyFour
        );
    }

    #[test]
    fn json() {
        let model = SUT::sample_iphone_unknown();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
        {
            "label": "Unknown Phone",
            "name": "Unknown Device Name",
            "model": "iPhone",
            "mnemonicWordCount": 24
        }
        "#,
        )
    }

    #[test]
    fn json_app_version_and_system_version_set() {
        let sut = SUT::new(
            DisplayName::new("New Phone").unwrap(),
            "My precious",
            "iPhone 15 Pro",
            "17.4.1".to_owned(),
            "1.6.0".to_owned(),
            "Apple".to_owned(),
            BIP39WordCount::TwentyFour,
        );
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
        {
            "label": "New Phone",
            "name": "My precious",
            "model": "iPhone 15 Pro",
            "systemVersion": "17.4.1",
            "hostAppVersion": "1.6.0",
            "hostVendor": "Apple",
            "mnemonicWordCount": 24
        }
        "#,
        )
    }
}
