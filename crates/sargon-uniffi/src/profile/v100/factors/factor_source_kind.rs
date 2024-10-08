use crate::prelude::*;
use sargon::FactorSourceKind as InternalFactorSourceKind;

/// The **kind** (or "type") of FactorSource describes how it is used.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
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
    LedgerHQHardwareWallet,

    /// A user owned mnemonic (and optional BIP39 passphrase) user has to input when used,
    /// e.g. during signing.
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (Mnemonic)
    OffDeviceMnemonic,

    /// A contact, friend, company, organization or otherwise third party the user trusts enough
    /// to be given a recovery token user has minted and sent the this contact.
    ///
    /// Attributes:
    ///  * **Not** mine
    ///  * Off device
    TrustedContact,

    /// An encrypted user owned mnemonic (*never* any BIP39 passphrase) which can
    /// be decrypted by answers to **security question**, which are personal questions
    /// that should be only known to the user.
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic  (**Encrypted** mnemonic)
    SecurityQuestions,

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
    ArculusCard,
}

impl From<InternalFactorSourceKind> for FactorSourceKind {
    fn from(kind: InternalFactorSourceKind) -> Self {
        match kind {
            InternalFactorSourceKind::Device => FactorSourceKind::Device,
            InternalFactorSourceKind::LedgerHQHardwareWallet => {
                FactorSourceKind::LedgerHQHardwareWallet
            }
            InternalFactorSourceKind::OffDeviceMnemonic => FactorSourceKind::OffDeviceMnemonic,
            InternalFactorSourceKind::TrustedContact => FactorSourceKind::TrustedContact,
            InternalFactorSourceKind::SecurityQuestions => FactorSourceKind::SecurityQuestions,
            InternalFactorSourceKind::ArculusCard => FactorSourceKind::ArculusCard,
        }
    }
}

impl Into<InternalFactorSourceKind> for FactorSourceKind {
    fn into(self) -> InternalFactorSourceKind {
        match self {
            FactorSourceKind::Device => InternalFactorSourceKind::Device,
            FactorSourceKind::LedgerHQHardwareWallet => InternalFactorSourceKind::LedgerHQHardwareWallet,
            FactorSourceKind::OffDeviceMnemonic => InternalFactorSourceKind::OffDeviceMnemonic,
            FactorSourceKind::TrustedContact => InternalFactorSourceKind::TrustedContact,
            FactorSourceKind::SecurityQuestions => InternalFactorSourceKind::SecurityQuestions,
            FactorSourceKind::ArculusCard => InternalFactorSourceKind::ArculusCard,
        }
    }
}

#[uniffi::export]
pub fn new_factor_source_kind_sample() -> FactorSourceKind {
    InternalFactorSourceKind::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_kind_sample_other() -> FactorSourceKind {
    InternalFactorSourceKind::sample_other().into()
}

#[uniffi::export]
pub fn new_factor_source_kind_from_string(
    string: String,
) -> Result<FactorSourceKind> {
    InternalFactorSourceKind::from_str(&string).map_result()
}

#[uniffi::export]
pub fn factor_source_kind_to_string(kind: FactorSourceKind) -> String {
    kind.into_internal().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceKind;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_factor_source_kind_sample(),
                new_factor_source_kind_sample_other(),
                // duplicates should get removed
                new_factor_source_kind_sample(),
                new_factor_source_kind_sample_other(),
            ])
            .len(),
            2
        );
    }

    #[test]
    fn string_roundtrip() {
        let sut = SUT::sample();
        let str = factor_source_kind_to_string(sut);
        let from_str = new_factor_source_kind_from_string(str).unwrap();
        assert_eq!(sut, from_str);
    }
}
