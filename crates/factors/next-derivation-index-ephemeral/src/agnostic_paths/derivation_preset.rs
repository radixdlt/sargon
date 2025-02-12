use crate::prelude::*;

/// Derivation Presets are Network agnostic and Index agnostic
/// "templates" for DerivationPaths.
#[derive(
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    enum_iterator::Sequence,
    derive_more::Debug,
)]
pub enum DerivationPreset {
    /// Used to form DerivationPaths used to derive FactorInstances
    /// for "veci": Virtual Entity Creating (Factor)Instance for accounts.
    /// `(EntityKind::Account, KeySpace::Unsecurified, KeyKind::TransactionSigning)`
    #[debug("A-VECI")]
    AccountVeci,

    /// Used to form DerivationPaths used to derive FactorInstances
    /// for "mfa" to securify accounts.
    /// `(EntityKind::Account, KeySpace::Securified, KeyKind::TransactionSigning)`
    #[debug("A-MFA")]
    AccountMfa,

    /// Used to form DerivationPaths used to derive FactorInstances
    /// for Authentication Signing (Securified) for accounts
    /// `(EntityKind::Account, KeySpace::Securified, KeyKind::AuthenticationSigning)`
    #[debug("A-Rola")]
    AccountRola,

    /// Used to form DerivationPaths used to derive FactorInstances
    /// for "veci": Virtual Entity Creating (Factor)Instance for personas.
    /// `(EntityKind::Identity, KeySpace::Unsecurified, KeyKind::TransactionSigning)`
    #[debug("I-VECI")]
    IdentityVeci,

    /// Used to form DerivationPaths used to derive FactorInstances
    /// for "mfa" to securify personas.
    /// `(EntityKind::Identity, KeySpace::Securified, KeyKind::TransactionSigning)`
    #[debug("I-MFA")]
    IdentityMfa,

    /// Used to form DerivationPaths used to derive FactorInstances
    /// for Authentication Signing (Securified) for personas
    /// `(EntityKind::Identity, KeySpace::Securified, KeyKind::AuthenticationSigning)`
    #[debug("I-Rola")]
    IdentityRola,
}

// =============
// Construction
// =============
impl DerivationPreset {
    /// All DerivationPreset's, used to fill cache.
    pub fn all() -> IndexSet<Self> {
        enum_iterator::all::<Self>().collect()
    }

    /// Selects a `DerivationPreset` for veci based on `CAP26EntityKind`,
    /// i.e. either `DerivationPreset::AccountVeci` or `DerivationPreset::IdentityVeci`.
    pub fn veci_entity_kind(entity_kind: CAP26EntityKind) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::AccountVeci,
            CAP26EntityKind::Identity => Self::IdentityVeci,
        }
    }

    /// Selects a `DerivationPreset` for MFA based on `CAP26EntityKind`,
    /// i.e. either `DerivationPreset::AccountMfa` or `DerivationPreset::IdentityMfa`.
    pub fn mfa_entity_kind(entity_kind: CAP26EntityKind) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::AccountMfa,
            CAP26EntityKind::Identity => Self::IdentityMfa,
        }
    }

    /// Selects a `DerivationPreset` for MFA based on `CAP26EntityKind`,
    /// i.e. either `DerivationPreset::AccountRola` or `DerivationPreset::IdentityRola`.
    pub fn rola_entity_kind(entity_kind: CAP26EntityKind) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::AccountRola,
            CAP26EntityKind::Identity => Self::IdentityRola,
        }
    }
}

// =============
// Instance Methods
// =============
impl DerivationPreset {
    /// Returns the `CAP26EntityKind` of the `DerivationPreset`.
    pub fn entity_kind(&self) -> CAP26EntityKind {
        match self {
            Self::AccountVeci | Self::AccountMfa | Self::AccountRola => {
                CAP26EntityKind::Account
            }
            Self::IdentityVeci | Self::IdentityMfa | Self::IdentityRola => {
                CAP26EntityKind::Identity
            }
        }
    }

    /// Returns the `CAP26KeyKind` of the `DerivationPreset`.
    pub fn key_kind(&self) -> CAP26KeyKind {
        match self {
            Self::AccountVeci
            | Self::IdentityVeci
            | Self::AccountMfa
            | Self::IdentityMfa => CAP26KeyKind::TransactionSigning,
            Self::AccountRola | Self::IdentityRola => {
                CAP26KeyKind::AuthenticationSigning
            }
        }
    }

    /// Returns the `KeySpace` of the `DerivationPreset`.
    pub fn key_space(&self) -> KeySpace {
        match self {
            Self::AccountVeci | Self::IdentityVeci => {
                KeySpace::Unsecurified {
                    // We never pre-derive keys for unhardened BIP44 paths.
                    is_hardened: true,
                }
            }
            Self::AccountMfa
            | Self::IdentityMfa
            | Self::AccountRola
            | Self::IdentityRola => KeySpace::Securified,
        }
    }

    /// Maps a DerivationPreset to a `IndexAgnosticPath` which is network aware.
    pub fn index_agnostic_path_on_network(
        &self,
        network_id: NetworkID,
    ) -> IndexAgnosticPath {
        IndexAgnosticPath::from((network_id, *self))
    }
}

impl HasSampleValues for DerivationPreset {
    fn sample() -> Self {
        DerivationPreset::AccountVeci
    }

    fn sample_other() -> Self {
        DerivationPreset::IdentityVeci
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DerivationPreset;

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
    fn test_mfa_entity_kind() {
        assert_eq!(
            SUT::mfa_entity_kind(CAP26EntityKind::Account),
            SUT::AccountMfa
        );
        assert_eq!(
            SUT::mfa_entity_kind(CAP26EntityKind::Identity),
            SUT::IdentityMfa
        );
    }

    #[test]
    fn test_rola_entity_kind() {
        assert_eq!(
            SUT::rola_entity_kind(CAP26EntityKind::Account),
            SUT::AccountRola
        );
        assert_eq!(
            SUT::rola_entity_kind(CAP26EntityKind::Identity),
            SUT::IdentityRola
        );
    }
}
