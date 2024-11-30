use crate::prelude::*;

impl FactorSourceID {
    pub(crate) fn sample_device() -> Self {
        FactorSourceIDFromHash::sample_device().into()
    }
    pub(crate) fn sample_ledger() -> Self {
        FactorSourceIDFromHash::sample_ledger().into()
    }
    pub(crate) fn sample_ledger_other() -> Self {
        FactorSourceIDFromHash::sample_ledger_other().into()
    }
    pub(crate) fn sample_arculus() -> Self {
        FactorSourceIDFromHash::sample_arculus().into()
    }
    pub(crate) fn sample_arculus_other() -> Self {
        FactorSourceIDFromHash::sample_arculus_other().into()
    }

    pub(crate) fn sample_password() -> Self {
        FactorSourceIDFromHash::sample_password().into()
    }

    pub(crate) fn sample_password_other() -> Self {
        FactorSourceIDFromHash::sample_password_other().into()
    }

    /// Radix Wallet (UI) calls this "passphrase"
    pub(crate) fn sample_off_device() -> Self {
        FactorSourceIDFromHash::sample_off_device().into()
    }
    /// Radix Wallet (UI) calls this "passphrase"
    pub(crate) fn sample_off_device_other() -> Self {
        FactorSourceIDFromHash::sample_off_device_other().into()
    }
    pub(crate) fn sample_security_questions() -> Self {
        FactorSourceIDFromHash::sample_security_questions().into()
    }
    pub(crate) fn sample_device_other() -> Self {
        FactorSourceIDFromHash::sample_device_other().into()
    }
    pub(crate) fn sample_security_questions_other() -> Self {
        FactorSourceIDFromHash::sample_security_questions_other().into()
    }
    pub(crate) fn sample_trusted_contact() -> Self {
        FactorSource::sample_trusted_contact_frank().id()
    }
    pub(crate) fn sample_trusted_contact_other() -> Self {
        FactorSource::sample_trusted_contact_grace().id()
    }
}

#[allow(dead_code)]
pub static ALL_FACTOR_SOURCE_ID_SAMPLES_INC_NON_HD: Lazy<[FactorSourceID; 14]> =
    Lazy::new(|| {
        [
            FactorSourceID::sample_device(),
            FactorSourceID::sample_ledger(),
            FactorSourceID::sample_ledger_other(),
            FactorSourceID::sample_arculus(),
            FactorSourceID::sample_arculus_other(),
            FactorSourceID::sample_password(),
            FactorSourceID::sample_password_other(),
            FactorSourceID::sample_off_device(),
            FactorSourceID::sample_off_device_other(),
            FactorSourceID::sample_security_questions(),
            FactorSourceID::sample_device_other(),
            FactorSourceID::sample_security_questions_other(),
            FactorSourceID::sample_trusted_contact(),
            FactorSourceID::sample_trusted_contact_other(),
        ]
    });
