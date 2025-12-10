use crate::prelude::*;

/// A discriminator type for the `DetailedManifestClass` enum.
#[derive(
    Clone, PartialEq, Eq, Hash, derive_more::Display, enum_iterator::Sequence,
)]
pub enum DetailedManifestClassKind {
    #[display("General")]
    General,

    #[display("Transfer")]
    Transfer,

    #[display("ValidatorClaim")]
    ValidatorClaim,

    #[display("ValidatorStake")]
    ValidatorStake,

    #[display("ValidatorUnstake")]
    ValidatorUnstake,

    #[display("AccountDepositSettingsUpdate")]
    AccountDepositSettingsUpdate,

    #[display("PoolContribution")]
    PoolContribution,

    #[display("PoolRedemption")]
    PoolRedemption,

    #[display("DeleteAccounts")]
    DeleteAccounts,

    #[display("SecurifyEntity")]
    SecurifyEntity,

    #[display("AccessControllerRecovery")]
    AccessControllerRecovery,

    #[display("AccessControllerConfirmTimedRecovery")]
    AccessControllerConfirmTimedRecovery,

    #[display("AccessControllerStopTimedRecovery")]
    AccessControllerStopTimedRecovery,
}

impl DetailedManifestClassKind {
    /// All DerivationPreset's, used to fill cache.
    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }
}

impl std::fmt::Debug for DetailedManifestClassKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string())
    }
}

impl HasSampleValues for DetailedManifestClassKind {
    fn sample() -> Self {
        DetailedManifestClassKind::General
    }

    fn sample_other() -> Self {
        DetailedManifestClassKind::Transfer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DetailedManifestClassKind;

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
    fn display() {
        let len_kinds = SUT::all().len();
        let len_strings = SUT::all()
            .iter()
            .map(|k| k.to_string())
            .collect::<IndexSet<_>>()
            .len();
        assert_eq!(len_kinds, len_strings);
    }
}
