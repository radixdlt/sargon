use crate::prelude::*;

/// The reason that `KeysCollector` interacted with the host in order to
/// derive keys.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeysCollectionReason {
    /// When the create account flow, initiates keys collection
    /// for account VECIs
    CreatingNewAccount,

    /// When the create persona flow, initiates keys collection
    /// for identity VECIs
    CreatingNewPersona,

    /// When applying a security shield to an account, initiates keys collection
    /// for account MFA
    SecurifyingAccount,

    /// When applying a security shield to a persona, initiates keys collection
    /// for identity MFA
    SecurifyingPersona,

    /// When adding a new factor source, initiates keys collection
    /// for collecting various factor instances.
    PreDerivingKeys,
}

impl KeysCollectionReason {
    pub fn new_for_creating(entity_kind: CAP26EntityKind) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::CreatingNewAccount,
            CAP26EntityKind::Identity => Self::CreatingNewPersona,
        }
    }

    pub fn new_for_securifying(entity_kind: CAP26EntityKind) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::SecurifyingAccount,
            CAP26EntityKind::Identity => Self::SecurifyingPersona,
        }
    }

    pub fn new_for_pre_derivation() -> Self {
        Self::PreDerivingKeys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = KeysCollectionReason;

    #[test]
    fn test_for_creating_account() {
        assert_eq!(
            SUT::new_for_creating(CAP26EntityKind::Account),
            SUT::CreatingNewAccount
        )
    }

    #[test]
    fn test_for_creating_persona() {
        assert_eq!(
            SUT::new_for_creating(CAP26EntityKind::Identity),
            SUT::CreatingNewPersona
        )
    }

    #[test]
    fn test_for_securifying_account() {
        assert_eq!(
            SUT::new_for_securifying(CAP26EntityKind::Account),
            SUT::SecurifyingAccount
        )
    }

    #[test]
    fn test_for_securifying_persona() {
        assert_eq!(
            SUT::new_for_securifying(CAP26EntityKind::Identity),
            SUT::SecurifyingPersona
        )
    }

    #[test]
    fn test_for_pre_deriving_keys() {
        assert_eq!(SUT::new_for_pre_derivation(), SUT::PreDerivingKeys)
    }
}
