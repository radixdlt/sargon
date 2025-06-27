use crate::prelude::*;
use sargon::FactorSourceKind as InternalFactorSourceKind;

/// The **kind** (or "type") of FactorSource describes how it is used.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
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

    /// Input key material for mnemonic (and optional BIP39 passphrase).
    ///
    /// Attributes:
    ///  * Mine
    ///  * Off device
    ///  * Hierarchical deterministic (IKM -> HKDF -> Mnemonic)
    Password,
}

delegate_display_debug_into!(FactorSourceKind, InternalFactorSourceKind);

impl FactorSourceKind {
    pub fn into_internal(&self) -> InternalFactorSourceKind {
        self.clone().into()
    }
}

impl From<InternalFactorSourceKind> for FactorSourceKind {
    fn from(value: InternalFactorSourceKind) -> Self {
        match value {
            InternalFactorSourceKind::Device => FactorSourceKind::Device,
            InternalFactorSourceKind::LedgerHQHardwareWallet => {
                FactorSourceKind::LedgerHQHardwareWallet
            }
            InternalFactorSourceKind::OffDeviceMnemonic => {
                FactorSourceKind::OffDeviceMnemonic
            }
            InternalFactorSourceKind::ArculusCard => {
                FactorSourceKind::ArculusCard
            }
            InternalFactorSourceKind::Password => FactorSourceKind::Password,
            InternalFactorSourceKind::TrustedContact => {
                panic!("TrustedContact not yet supported in the Wallet")
            }
            InternalFactorSourceKind::SecurityQuestions => {
                panic!("SecurityQuestions not yet supported in the Wallet")
            }
        }
    }
}

impl From<FactorSourceKind> for InternalFactorSourceKind {
    fn from(val: FactorSourceKind) -> Self {
        match val {
            FactorSourceKind::Device => InternalFactorSourceKind::Device,
            FactorSourceKind::LedgerHQHardwareWallet => {
                InternalFactorSourceKind::LedgerHQHardwareWallet
            }
            FactorSourceKind::OffDeviceMnemonic => {
                InternalFactorSourceKind::OffDeviceMnemonic
            }
            FactorSourceKind::ArculusCard => {
                InternalFactorSourceKind::ArculusCard
            }
            FactorSourceKind::Password => InternalFactorSourceKind::Password,
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
    InternalFactorSourceKind::from_str(&string).into_result()
}

#[uniffi::export]
pub fn factor_source_kind_to_string(kind: FactorSourceKind) -> String {
    kind.into_internal().to_string()
}
