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
    Ord,
    uniffi::Enum,
)]
pub enum FactorSourceKind {
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

    /// A user owned mnemonic (and optional BIP39 passphrase) user has to input when used,
    /// e.g. during signing.
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (Mnemonic)
    #[serde(rename = "offDeviceMnemonic")]
    OffDeviceMnemonic,

    /// A contact, friend, company, organization or otherwise third party the user trusts enough
    /// to be given a recovery token user has minted and sent the this contact.
    ///
    /// Attributes:
    ///  * **Not** mine
    ///  * Off device
    #[serde(rename = "trustedContact")]
    TrustedContact,

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

impl std::fmt::Display for FactorSourceKind {
    #[cfg(not(tarpaulin_include))] // false negative
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.discriminant())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(FactorSourceKind::Device, FactorSourceKind::Device);
        assert_eq!(
            FactorSourceKind::LedgerHQHardwareWallet,
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }
    #[test]
    fn inequality() {
        assert_ne!(
            FactorSourceKind::Device,
            FactorSourceKind::LedgerHQHardwareWallet
        );
    }

    #[test]
    fn hash() {
        assert_eq!(
            BTreeSet::from_iter(
                [FactorSourceKind::Device, FactorSourceKind::Device]
                    .into_iter()
            )
            .len(),
            1
        );
    }

    #[test]
    fn ord() {
        assert!(FactorSourceKind::Device < FactorSourceKind::TrustedContact);
    }

    #[test]
    fn discriminant() {
        assert_eq!(FactorSourceKind::Device.discriminant(), "device");
        assert_eq!(
            FactorSourceKind::SecurityQuestions.discriminant(),
            "securityQuestions"
        );
        assert_eq!(
            FactorSourceKind::LedgerHQHardwareWallet.discriminant(),
            "ledgerHQHardwareWallet"
        );
        assert_eq!(
            FactorSourceKind::OffDeviceMnemonic.discriminant(),
            "offDeviceMnemonic"
        );

        assert_eq!(
            FactorSourceKind::TrustedContact.discriminant(),
            "trustedContact"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", FactorSourceKind::Device.discriminant()),
            "device"
        );
        assert_eq!(
            format!(
                "{}",
                FactorSourceKind::LedgerHQHardwareWallet.discriminant()
            ),
            "ledgerHQHardwareWallet"
        );
        assert_eq!(
            format!("{}", FactorSourceKind::SecurityQuestions.discriminant()),
            "securityQuestions"
        );
        assert_eq!(
            format!("{}", FactorSourceKind::OffDeviceMnemonic.discriminant()),
            "offDeviceMnemonic"
        );
        assert_eq!(
            format!("{}", FactorSourceKind::TrustedContact.discriminant()),
            "trustedContact"
        );
    }

    #[test]
    fn json_roundtrip() {
        assert_json_value_eq_after_roundtrip(
            &FactorSourceKind::TrustedContact,
            json!("trustedContact"),
        );
        assert_json_value_eq_after_roundtrip(
            &FactorSourceKind::Device,
            json!("device"),
        );
        assert_json_value_eq_after_roundtrip(
            &FactorSourceKind::SecurityQuestions,
            json!("securityQuestions"),
        );
        assert_json_value_eq_after_roundtrip(
            &FactorSourceKind::LedgerHQHardwareWallet,
            json!("ledgerHQHardwareWallet"),
        );
        assert_json_value_eq_after_roundtrip(
            &FactorSourceKind::OffDeviceMnemonic,
            json!("offDeviceMnemonic"),
        );
        assert_json_roundtrip(&FactorSourceKind::Device);
    }
}
