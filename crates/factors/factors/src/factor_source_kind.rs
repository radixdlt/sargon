use crate::prelude::*;

/// The **kind** (or "type") of FactorSource describes how it is used.
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    enum_iterator::Sequence,
    Ord,
)]
pub enum FactorSourceKind {
    /// A user owned hardware wallet by vendor Ledger HQ, most commonly
    /// a Ledger Nano S or Ledger Nano X. Less common models are Ledger Nano S Plus
    /// Ledger Stax.
    ///
    /// Attributes:
    /// * Mine
    /// * Off device
    /// * Hardware (requires Browser Connector Extension to communicate with wallet)
    /// * Hierarchical deterministic
    /// * Entity creating (accounts only)
    #[serde(rename = "ledgerHQHardwareWallet")]
    LedgerHQHardwareWallet,

    /// An Arculus card, in credit card size, communicating with host using NFC.
    ///
    /// For more info see [link]
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (**Encrypted** mnemonic)\
    ///  * Hardware (communicates with host using NFC)
    ///
    /// [link]: https://www.getarculus.com/
    #[serde(rename = "arculusCard")]
    ArculusCard,

    /// Input key material for mnemonic (and optional BIP39 passphrase).
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic (IKM -> HKDF -> Mnemonic)
    #[serde(rename = "password")]
    Password,

    /// An encrypted user owned mnemonic (*never* any BIP39 passphrase) which can
    /// be decrypted by answers to **security question**, which are personal questions
    /// that should be only known to the user.
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (**Encrypted** mnemonic)
    #[serde(rename = "securityQuestions")]
    SecurityQuestions,

    /// A user owned mnemonic (and optional BIP39 passphrase) user has to input when used,
    /// e.g. during signing.
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (Mnemonic)
    #[serde(rename = "offDeviceMnemonic")]
    OffDeviceMnemonic,

    /// A user owned unencrypted mnemonic (and optional BIP39 passphrase) stored on device,
    /// thus directly usable. This kind is used as the standard factor source for all new
    /// wallet users.
    ///
    /// Attributes:
    /// * Mine
    /// * On device
    /// * Hierarchical deterministic (Mnemonic)
    /// * Entity creating
    #[serde(rename = "device")]
    Device,

    /// A contact, friend, company, organization or otherwise third party the user trusts enough
    /// to be given a recovery token user has minted and sent the this contact.
    ///
    /// Attributes:
    ///  * **Not** mine
    ///  * Off device
    #[serde(rename = "trustedContact")]
    TrustedContact,
}

impl FactorSourceKind {
    /// All FactorSourceKind
    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }
}

impl FactorSourceKind {
    pub fn discriminant(&self) -> String {
        // We do `to_value.as_str` instead of `to_string(_pretty)` to avoid unwanted quotation marks around the string.
        serde_json::to_value(self)
            .expect("Should always be able to JSON encode FactorSourceKind.")
            .as_str()
            .expect("Representation should always be string")
            .to_owned()
    }
}

impl FactorSourceKind {
    pub fn category(&self) -> FactorSourceCategory {
        use FactorSourceCategory::*;
        match self {
            Self::LedgerHQHardwareWallet | Self::ArculusCard => Hardware,
            Self::Password
            | Self::SecurityQuestions
            | Self::OffDeviceMnemonic => Information,
            Self::Device => Identity,
            Self::TrustedContact => Contact,
        }
    }

    pub fn is_supported(&self) -> bool {
        match self {
            Self::LedgerHQHardwareWallet
            | Self::ArculusCard
            | Self::Password
            | Self::OffDeviceMnemonic
            | Self::Device => true,
            Self::SecurityQuestions | Self::TrustedContact => false,
        }
    }
}

impl FactorSourceKind {
    pub fn display_order_for_primary_threshold_selection(&self) -> u8 {
        match self {
            FactorSourceKind::Device => 0,
            FactorSourceKind::ArculusCard => 1,
            FactorSourceKind::LedgerHQHardwareWallet => 2,
            FactorSourceKind::Password => 3,
            FactorSourceKind::OffDeviceMnemonic => 4,
            FactorSourceKind::TrustedContact => 5,
            FactorSourceKind::SecurityQuestions => 6,
        }
    }
}

impl HasSampleValues for FactorSourceKind {
    fn sample() -> Self {
        Self::Device
    }

    fn sample_other() -> Self {
        Self::LedgerHQHardwareWallet
    }
}

impl std::fmt::Display for FactorSourceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.discriminant())
    }
}

impl FromStr for FactorSourceKind {
    type Err = CommonError;
    fn from_str(s: &str) -> Result<Self> {
        let value = serde_json::Value::String(s.to_owned());
        serde_json::from_value(value).map_err(|_| {
            CommonError::InvalidFactorSourceKind {
                bad_value: s.to_owned(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceKind;

    #[test]
    fn ord() {
        assert!(SUT::Device < SUT::TrustedContact);
        let unsorted = SUT::all(); // is in fact sorted
        let mut sorted = unsorted.clone();
        sorted.sort();
        assert_eq!(unsorted, sorted);
    }

    #[test]
    fn string_roundtrip() {
        use FactorSourceKind::*;
        let eq = |f: SUT, s| {
            assert_eq!(f.to_string(), s);
            assert_eq!(SUT::from_str(s).unwrap(), f);
        };

        eq(Device, "device");
        eq(LedgerHQHardwareWallet, "ledgerHQHardwareWallet");
        eq(OffDeviceMnemonic, "offDeviceMnemonic");
        eq(TrustedContact, "trustedContact");
        eq(SecurityQuestions, "securityQuestions");
        eq(Password, "password");
    }

    #[test]
    fn from_str_err() {
        let s = "invalid factor source kind!";
        assert_eq!(
            SUT::from_str(s),
            Err(CommonError::InvalidFactorSourceKind {
                bad_value: s.to_owned(),
            })
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

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter([SUT::Device, SUT::Device].into_iter()).len(),
            1
        );
    }

    #[test]
    fn discriminant() {
        assert_eq!(SUT::Device.discriminant(), "device");
        assert_eq!(SUT::SecurityQuestions.discriminant(), "securityQuestions");
        assert_eq!(
            SUT::LedgerHQHardwareWallet.discriminant(),
            "ledgerHQHardwareWallet"
        );
        assert_eq!(SUT::OffDeviceMnemonic.discriminant(), "offDeviceMnemonic");

        assert_eq!(SUT::TrustedContact.discriminant(), "trustedContact");
        assert_eq!(SUT::Password.discriminant(), "password");
    }

    #[test]
    fn category() {
        assert_eq!(
            SUT::LedgerHQHardwareWallet.category(),
            FactorSourceCategory::Hardware
        );
        assert_eq!(SUT::ArculusCard.category(), FactorSourceCategory::Hardware);
        assert_eq!(SUT::Password.category(), FactorSourceCategory::Information);
        assert_eq!(
            SUT::SecurityQuestions.category(),
            FactorSourceCategory::Information
        );
        assert_eq!(
            SUT::OffDeviceMnemonic.category(),
            FactorSourceCategory::Information
        );
        assert_eq!(SUT::Device.category(), FactorSourceCategory::Identity);
        assert_eq!(
            SUT::TrustedContact.category(),
            FactorSourceCategory::Contact
        );
    }

    #[test]
    fn is_supported() {
        assert!(SUT::Device.is_supported());
        assert!(SUT::LedgerHQHardwareWallet.is_supported());
        assert!(SUT::ArculusCard.is_supported());
        assert!(SUT::OffDeviceMnemonic.is_supported());
        assert!(SUT::Password.is_supported());
        assert!(!SUT::TrustedContact.is_supported());
        assert!(!SUT::SecurityQuestions.is_supported());
    }

    #[test]
    fn display_order_for_primary_threshold_selection() {
        assert_eq!(
            SUT::Device.display_order_for_primary_threshold_selection(),
            0
        );
        assert_eq!(
            SUT::ArculusCard.display_order_for_primary_threshold_selection(),
            1
        );
        assert_eq!(
            SUT::LedgerHQHardwareWallet
                .display_order_for_primary_threshold_selection(),
            2
        );
        assert_eq!(
            SUT::Password.display_order_for_primary_threshold_selection(),
            3
        );
        assert_eq!(
            SUT::OffDeviceMnemonic
                .display_order_for_primary_threshold_selection(),
            4
        );
        assert_eq!(
            SUT::TrustedContact.display_order_for_primary_threshold_selection(),
            5
        );
        assert_eq!(
            SUT::SecurityQuestions
                .display_order_for_primary_threshold_selection(),
            6
        );
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", SUT::Device.discriminant()), "device");
        assert_eq!(
            format!("{}", SUT::LedgerHQHardwareWallet.discriminant()),
            "ledgerHQHardwareWallet"
        );
        assert_eq!(
            format!("{}", SUT::SecurityQuestions.discriminant()),
            "securityQuestions"
        );
        assert_eq!(
            format!("{}", SUT::OffDeviceMnemonic.discriminant()),
            "offDeviceMnemonic"
        );
        assert_eq!(
            format!("{}", SUT::TrustedContact.discriminant()),
            "trustedContact"
        );
        assert_eq!(format!("{}", SUT::Password.discriminant()), "password");
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &SUT::TrustedContact,
            json!("trustedContact"),
        );
        assert_json_value_eq_after_roundtrip(&SUT::Device, json!("device"));
        assert_json_value_eq_after_roundtrip(
            &SUT::SecurityQuestions,
            json!("securityQuestions"),
        );
        assert_json_value_eq_after_roundtrip(
            &SUT::LedgerHQHardwareWallet,
            json!("ledgerHQHardwareWallet"),
        );
        assert_json_value_eq_after_roundtrip(
            &SUT::OffDeviceMnemonic,
            json!("offDeviceMnemonic"),
        );
        assert_json_value_eq_after_roundtrip(&SUT::Password, json!("password"));
        assert_json_roundtrip(&SUT::Device);
    }
}
