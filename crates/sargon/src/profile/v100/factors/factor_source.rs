use crate::prelude::*;

#[derive(
    Serialize,
    Deserialize,
    Clone,
    EnumAsInner,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
#[serde(untagged, remote = "Self")]
pub enum FactorSource {
    Device {
        #[serde(rename = "device")]
        #[display("DeviceFS({value})")]
        value: DeviceFactorSource,
    },

    Ledger {
        #[serde(rename = "ledgerHQHardwareWallet")]
        #[display("LedgerHWFS({value})")]
        value: LedgerHardwareWalletFactorSource,
    },

    OffDeviceMnemonic {
        #[serde(rename = "offDeviceMnemonic")]
        #[display("OffDevice({value})")]
        value: OffDeviceMnemonicFactorSource,
    },

    ArculusCard {
        #[serde(rename = "arculusCard")]
        #[display("ArculusCard({value})")]
        value: ArculusCardFactorSource,
    },

    SecurityQuestions {
        #[serde(rename = "securityQuestions")]
        #[display("SecurityQuestions({value})")]
        value: SecurityQuestions_NOT_PRODUCTION_READY_FactorSource,
    },

    TrustedContact {
        #[serde(rename = "trustedContact")]
        #[display("TrustedContact({value})")]
        value: TrustedContactFactorSource,
    },

    Password {
        #[serde(rename = "password")]
        #[display("Password({value})")]
        value: PasswordFactorSource,
    },
}

/// A bit hacky... but used to make it possible for us to validate FactorInstance
/// in RoleWithFactor...
impl IsMaybeKeySpaceAware for FactorSource {
    fn maybe_key_space(&self) -> Option<KeySpace> {
        None
    }
}

impl BaseIsFactorSource for FactorSource {
    fn set_common_properties(&mut self, updated: FactorSourceCommon) {
        match self {
            FactorSource::Device { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::Ledger { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::SecurityQuestions { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::ArculusCard { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::OffDeviceMnemonic { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::TrustedContact { value } => {
                value.set_common_properties(updated)
            }
            FactorSource::Password { value } => {
                value.set_common_properties(updated)
            }
        }
    }

    fn common_properties(&self) -> FactorSourceCommon {
        match self {
            FactorSource::Device { value } => value.common_properties(),
            FactorSource::Ledger { value } => value.common_properties(),
            FactorSource::SecurityQuestions { value } => {
                value.common_properties()
            }
            FactorSource::ArculusCard { value } => value.common_properties(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.common_properties()
            }
            FactorSource::TrustedContact { value } => value.common_properties(),
            FactorSource::Password { value } => value.common_properties(),
        }
    }

    fn factor_source_kind(&self) -> FactorSourceKind {
        match self {
            FactorSource::Device { value } => value.factor_source_kind(),
            FactorSource::Ledger { value } => value.factor_source_kind(),
            FactorSource::SecurityQuestions { value } => {
                value.factor_source_kind()
            }
            FactorSource::ArculusCard { value } => value.factor_source_kind(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.factor_source_kind()
            }
            FactorSource::TrustedContact { value } => {
                value.factor_source_kind()
            }
            FactorSource::Password { value } => value.factor_source_kind(),
        }
    }

    fn factor_source_id(&self) -> FactorSourceID {
        match self {
            FactorSource::Device { value } => value.factor_source_id(),
            FactorSource::Ledger { value } => value.factor_source_id(),
            FactorSource::SecurityQuestions { value } => {
                value.factor_source_id()
            }
            FactorSource::ArculusCard { value } => value.factor_source_id(),
            FactorSource::OffDeviceMnemonic { value } => {
                value.factor_source_id()
            }
            FactorSource::TrustedContact { value } => value.factor_source_id(),
            FactorSource::Password { value } => value.factor_source_id(),
        }
    }

    fn name(&self) -> String {
        match self {
            FactorSource::Device { value } => value.name(),
            FactorSource::Ledger { value } => value.name(),
            FactorSource::SecurityQuestions { value } => value.name(),
            FactorSource::ArculusCard { value } => value.name(),
            FactorSource::OffDeviceMnemonic { value } => value.name(),
            FactorSource::TrustedContact { value } => value.name(),
            FactorSource::Password { value } => value.name(),
        }
    }
}

impl Identifiable for FactorSource {
    type ID = FactorSourceID;

    fn id(&self) -> Self::ID {
        self.factor_source_id()
    }
}

impl From<DeviceFactorSource> for FactorSource {
    fn from(value: DeviceFactorSource) -> Self {
        FactorSource::Device { value }
    }
}

impl From<LedgerHardwareWalletFactorSource> for FactorSource {
    fn from(value: LedgerHardwareWalletFactorSource) -> Self {
        FactorSource::Ledger { value }
    }
}

impl FactorSource {
    pub fn is_main_bdfs(&self) -> bool {
        if let Some(dfs) = self.as_device() {
            dfs.is_main_bdfs()
        } else {
            false
        }
    }
}

impl PartialOrd for FactorSource {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FactorSource {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.factor_source_kind().cmp(&other.factor_source_kind()) {
            Ordering::Equal => {}
            ord => return ord,
        }

        let self_last_used = self.common_properties().last_used_on;
        let other_last_used = &other.common_properties().last_used_on;
        match self_last_used.cmp(other_last_used) {
            Ordering::Equal => {}
            ord => return ord,
        }

        Ordering::Equal
    }
}

impl<'de> Deserialize<'de> for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        // https://github.com/serde-rs/serde/issues/1343#issuecomment-409698470
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            discriminator: String,
            #[serde(flatten, with = "FactorSource")]
            factor: FactorSource,
        }
        Wrapper::deserialize(deserializer).map(|w| w.factor)
    }
}

impl Serialize for FactorSource {
    #[cfg(not(tarpaulin_include))] // false negative
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("FactorSource", 2)?;
        let discriminator_key = "discriminator";
        match self {
            FactorSource::Device { value } => {
                let discriminant = "device";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::Ledger { value } => {
                let discriminant = "ledgerHQHardwareWallet";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::ArculusCard { value } => {
                let discriminant = "arculusCard";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::OffDeviceMnemonic { value } => {
                let discriminant = "offDeviceMnemonic";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::SecurityQuestions { value } => {
                let discriminant = "securityQuestions";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::TrustedContact { value } => {
                let discriminant = "trustedContact";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
            FactorSource::Password { value } => {
                let discriminant = "password";
                state.serialize_field(discriminator_key, discriminant)?;
                state.serialize_field(discriminant, value)?;
            }
        }
        state.end()
    }
}

impl HasSampleValues for FactorSource {
    fn sample() -> Self {
        Self::sample_device()
    }

    fn sample_other() -> Self {
        Self::sample_ledger()
    }
}
impl FactorSources {
    pub fn sample_values_all_with_filter(
        filter: impl Fn(&FactorSource) -> bool,
    ) -> Self {
        Self::from_iter(
            FactorSource::sample_values_all().into_iter().filter(filter),
        )
    }
    pub fn sample_values_all() -> Self {
        Self::sample_values_all_with_filter(|_| true)
    }
    pub fn sample_values_all_hd() -> Self {
        Self::sample_values_all_with_filter(|f| f.factor_source_id().is_hash())
    }
}
impl FactorSource {
    pub fn sample_values_all() -> Vec<Self> {
        vec![
            Self::sample_device_babylon(),
            Self::sample_device_babylon_other(),
            Self::sample_device_olympia(),
            Self::sample_ledger(),
            Self::sample_ledger_other(),
            Self::sample_arculus(),
            Self::sample_arculus_other(),
            Self::sample_off_device(),
            Self::sample_off_device_other(),
            Self::sample_trusted_contact_frank(),
            Self::sample_trusted_contact_grace(),
            Self::sample_trusted_contact_judy(),
            Self::sample_trusted_contact_oscar(),
            Self::sample_trusted_contact_trudy(),
            Self::sample_trusted_contact_radix(),
            Self::sample_security_questions(),
            Self::sample_security_questions_other(),
            Self::sample_password(),
            Self::sample_password_other(),
        ]
    }
    pub fn sample_device() -> Self {
        Self::sample_device_babylon()
    }

    pub fn sample_device_babylon() -> Self {
        Self::from(DeviceFactorSource::sample_babylon())
    }

    pub fn sample_device_babylon_other() -> Self {
        Self::from(DeviceFactorSource::sample_babylon_other())
    }
    pub fn sample_device_olympia() -> Self {
        Self::from(DeviceFactorSource::sample_olympia())
    }

    pub fn sample_ledger() -> Self {
        Self::from(LedgerHardwareWalletFactorSource::sample())
    }
    pub fn sample_ledger_other() -> Self {
        Self::from(LedgerHardwareWalletFactorSource::sample_other())
    }

    pub fn sample_arculus() -> Self {
        Self::from(ArculusCardFactorSource::sample())
    }
    pub fn sample_arculus_other() -> Self {
        Self::from(ArculusCardFactorSource::sample_other())
    }

    pub fn sample_off_device() -> Self {
        Self::from(OffDeviceMnemonicFactorSource::sample())
    }
    pub fn sample_off_device_other() -> Self {
        Self::from(OffDeviceMnemonicFactorSource::sample_other())
    }

    pub fn sample_trusted_contact_frank() -> Self {
        Self::from(TrustedContactFactorSource::sample_frank())
    }
    pub fn sample_trusted_contact_grace() -> Self {
        Self::from(TrustedContactFactorSource::sample_grace())
    }
    pub fn sample_trusted_contact_judy() -> Self {
        Self::from(TrustedContactFactorSource::sample_judy())
    }
    pub fn sample_trusted_contact_oscar() -> Self {
        Self::from(TrustedContactFactorSource::sample_oscar())
    }
    pub fn sample_trusted_contact_trudy() -> Self {
        Self::from(TrustedContactFactorSource::sample_trudy())
    }
    pub fn sample_trusted_contact_radix() -> Self {
        Self::from(TrustedContactFactorSource::sample_radix())
    }

    pub fn sample_security_questions() -> Self {
        Self::from(SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample())
    }
    pub fn sample_security_questions_other() -> Self {
        Self::from(
            SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::sample_other(),
        )
    }

    pub fn sample_password() -> Self {
        Self::from(PasswordFactorSource::sample())
    }

    pub fn sample_password_other() -> Self {
        Self::from(PasswordFactorSource::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSource;

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
    fn device_common_properties() {
        assert_eq!(
            SUT::sample().common_properties(),
            DeviceFactorSource::sample_babylon().common
        );

        assert_eq!(
            SUT::sample_other().common_properties(),
            LedgerHardwareWalletFactorSource::sample().common
        )
    }

    #[test]
    fn name() {
        assert_eq!(SUT::sample().name(), DeviceFactorSource::sample().name());

        assert_eq!(
            SUT::sample_other().name(),
            LedgerHardwareWalletFactorSource::sample().name()
        )
    }

    #[test]
    fn factor_source_id_device() {
        assert_eq!(
            SUT::sample_device().factor_source_id(),
            DeviceFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_id_ledger() {
        assert_eq!(
            SUT::sample_ledger().factor_source_id(),
            LedgerHardwareWalletFactorSource::sample().factor_source_id()
        );
    }

    #[test]
    fn factor_source_kind_device() {
        assert_eq!(
            SUT::sample_device().factor_source_kind(),
            FactorSourceKind::Device
        );
    }

    #[test]
    fn get_set_common() {
        let test = |sut: SUT| {
            let mut sut = sut;
            let mut common = sut.common_properties();
            let new_date = Timestamp::now_utc();
            common.last_used_on = new_date;
            sut.set_common_properties(common);
            assert_eq!(sut.common_properties().last_used_on, new_date);
        };
        FactorSource::sample_values_all().into_iter().for_each(test);
    }

    #[test]
    fn factor_source_kind_ledger() {
        assert_eq!(
            SUT::sample_ledger().factor_source_kind(),
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }

    #[test]
    fn factor_source_kind_security_questions() {
        assert_eq!(
            SUT::sample_security_questions().factor_source_kind(),
            FactorSourceKind::SecurityQuestions
        );
    }

    #[test]
    fn factor_source_kind_arculus_card() {
        assert_eq!(
            SUT::sample_arculus().factor_source_kind(),
            FactorSourceKind::ArculusCard
        );
    }

    #[test]
    fn factor_source_kind_off_device_mnemonic() {
        assert_eq!(
            SUT::sample_off_device().factor_source_kind(),
            FactorSourceKind::OffDeviceMnemonic
        );
    }

    #[test]
    fn factor_source_kind_trusted_contact() {
        assert_eq!(
            SUT::sample_trusted_contact_frank().factor_source_kind(),
            FactorSourceKind::TrustedContact
        );
    }

    #[test]
    fn factor_source_kind_password() {
        assert_eq!(
            SUT::sample_password().factor_source_kind(),
            FactorSourceKind::Password
        );
    }

    #[test]
    fn into_from_device() {
        let factor_source: SUT = DeviceFactorSource::sample().into();
        assert_eq!(
            factor_source,
            SUT::Device {
                value: DeviceFactorSource::sample()
            }
        );
    }

    #[test]
    fn into_from_ledger() {
        let factor_source: SUT =
            LedgerHardwareWalletFactorSource::sample().into();
        assert_eq!(
            factor_source,
            SUT::Ledger {
                value: LedgerHardwareWalletFactorSource::sample()
            }
        );
    }

    #[test]
    fn json_roundtrip_device() {
        let model = SUT::sample_device();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "device",
                "device": {
                    "id": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                    },
                    "common": {
                        "flags": ["main"],
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "lastUsedOn": "2023-09-11T16:05:56.000Z"
                    },
                    "hint": {
                        "name": "My precious",
                        "model": "iPhone SE 2nd gen",
                        "mnemonicWordCount": 24,
                        "systemVersion": "iOS 17.4.1",
                        "hostAppVersion": "1.6.4",
                        "hostVendor": "Apple"
                    }
                }
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_device_without_host_info() {
        let mut device = DeviceFactorSource::sample_babylon();
        device.hint.system_version = None;
        device.hint.host_app_version = None;
        device.hint.host_vendor = None;
        let model = FactorSource::from(device);

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "device",
                "device": {
                    "id": {
                        "kind": "device",
                        "body": "f1a93d324dd0f2bff89963ab81ed6e0c2ee7e18c0827dc1d3576b2d9f26bbd0a"
                    },
                    "common": {
                        "flags": ["main"],
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "lastUsedOn": "2023-09-11T16:05:56.000Z"
                    },
                    "hint": {
                        "name": "My precious",
                        "model": "iPhone SE 2nd gen",
                        "mnemonicWordCount": 24
                    }
                }
            }
            "#,
        )
    }

    #[test]
    fn json_roundtrip_ledger() {
        let model = SUT::sample_ledger();
        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "discriminator": "ledgerHQHardwareWallet",
                "ledgerHQHardwareWallet": {
                    "id": {
                        "kind": "ledgerHQHardwareWallet",
                        "body": "ab59987eedd181fe98e512c1ba0f5ff059f11b5c7c56f15614dcc9fe03fec58b"
                    },
                    "common": {
                        "addedOn": "2023-09-11T16:05:56.000Z",
                        "cryptoParameters": {
                            "supportedCurves": ["curve25519"],
                            "supportedDerivationPathSchemes": ["cap26"]
                        },
                        "flags": [],
                        "lastUsedOn": "2023-09-11T16:05:56.000Z"
                    },
                    "hint": {
                        "name": "Orange, scratched",
                        "model": "nanoS+"
                    }
                }
            }
            "#,
        )
    }
}
