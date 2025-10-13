use crate::prelude::*;

/// A "template" FactorSourceID/FactorSource to be used in a RoleTemplate is
/// FactorSourceKind with some placeholder ID, to distinguish between two different
/// FactorSourceIDs of some kind, e.g. `FactorSourceID::sample()` and `FactorSourceID::sample_other()`.
/// but exactly which FactorSourceID values are not yet known, since this is a template.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FactorSourceTemplate {
    /// The kind of FactorSource, e.g. Device, LedgerHQHardwareWallet, Password, etc.
    pub kind: FactorSourceKind,

    /// Some placeholder ID to distinguish between two different FactorSourceIDs
    /// to be concretely defined later.
    pub id: u8,
}

pub(crate) type RoleTemplate<const ROLE: u8> =
    AbstractBuiltRoleWithFactor<ROLE, FactorSourceTemplate>;

pub type PrimaryRoleTemplate = RoleTemplate<{ ROLE_PRIMARY }>;
pub type RecoveryRoleTemplate = RoleTemplate<{ ROLE_RECOVERY }>;
pub type ConfirmationRoleTemplate = RoleTemplate<{ ROLE_CONFIRMATION }>;

impl PrimaryRoleTemplate {
    pub(crate) fn new(
        threshold_factors: impl IntoIterator<Item = FactorSourceTemplate>,
    ) -> Self {
        let threshold_factors = threshold_factors.into_iter().collect_vec();
        Self::with_factors_and_threshold(Threshold::All, threshold_factors, [])
    }
}

impl RecoveryRoleTemplate {
    pub(crate) fn new(
        override_factors: impl IntoIterator<Item = FactorSourceTemplate>,
    ) -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], override_factors)
    }
}

impl ConfirmationRoleTemplate {
    pub(crate) fn new(
        override_factors: impl IntoIterator<Item = FactorSourceTemplate>,
    ) -> Self {
        Self::with_factors_and_threshold(Threshold::All, [], override_factors)
    }
}

impl FactorSourceTemplate {
    pub fn new(kind: FactorSourceKind, id: u8) -> Self {
        Self { kind, id }
    }

    pub fn device() -> Self {
        Self::new(FactorSourceKind::Device, 0)
    }

    fn ledger_id(id: u8) -> Self {
        Self::new(FactorSourceKind::LedgerHQHardwareWallet, id)
    }
    pub fn ledger() -> Self {
        Self::ledger_id(0)
    }

    pub fn ledger_other() -> Self {
        Self::ledger_id(1)
    }

    pub fn arculus() -> Self {
        Self::new(FactorSourceKind::ArculusCard, 0)
    }

    pub fn arculus_other() -> Self {
        Self::new(FactorSourceKind::ArculusCard, 1)
    }

    fn password_id(id: u8) -> Self {
        Self::new(FactorSourceKind::Password, id)
    }
    pub fn password() -> Self {
        Self::password_id(0)
    }
    pub fn password_other() -> Self {
        Self::password_id(1)
    }

    /// Radix Wallet (UI) calls this "Passphrase"
    pub fn off_device_mnemonic() -> Self {
        Self::new(FactorSourceKind::OffDeviceMnemonic, 0)
    }

    fn trusted_contact_id(id: u8) -> Self {
        Self::new(FactorSourceKind::TrustedContact, id)
    }

    pub fn trusted_contact() -> Self {
        Self::trusted_contact_id(0)
    }

    pub fn trusted_contact_other() -> Self {
        Self::trusted_contact_id(1)
    }

    pub fn security_questions() -> Self {
        Self::new(FactorSourceKind::SecurityQuestions, 0)
    }
}

impl IsMaybeKeySpaceAware for FactorSourceTemplate {
    fn maybe_key_space(&self) -> Option<KeySpace> {
        None
    }
}
