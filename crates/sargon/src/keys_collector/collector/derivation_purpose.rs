use crate::prelude::*;

/// The purpose that initiated an interaction with the host to derive keys.
/// The orchestrator behind this operation is the `KeysCollector`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DerivationPurpose {
    /// When the create account flow, initiates keys collection
    /// for account VECIs
    CreatingNewAccount,

    /// When the create persona flow, initiates keys collection
    /// for identity VECIs
    CreatingNewPersona,

    /// When applying a security shield to accounts and personas mixed, initiates keys collection
    /// for account MFA
    SecurifyingAccountsAndPersonas,

    /// When applying a security shield to only accounts, initiates keys collection
    /// for account MFA
    SecurifyingAccount,

    /// When applying a security shield to only personas, initiates keys collection
    /// for identity MFA
    SecurifyingPersona,

    /// When adding a new factor source, initiates keys collection
    /// for collecting various factor instances.
    PreDerivingKeys,
}

impl DerivationPurpose {
    pub fn creation_of_new_virtual_entity(
        entity_kind: CAP26EntityKind,
    ) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => Self::CreatingNewAccount,
            CAP26EntityKind::Identity => Self::CreatingNewPersona,
        }
    }

    pub fn for_securifying_or_updating(
        addresses_of_entities: &IndexSet<AddressOfAccountOrPersona>,
    ) -> Self {
        let account_addresses = addresses_of_entities
            .iter()
            .filter(|a| a.is_account())
            .collect_vec();
        let identity_addresses = addresses_of_entities
            .iter()
            .filter(|a| a.is_identity())
            .collect_vec();

        match (account_addresses.is_empty(), identity_addresses.is_empty()) {
            (true, true) => unreachable!("Incorrect implementation"), // weird!
            (true, false) => Self::SecurifyingPersona,
            (false, false) => Self::SecurifyingAccountsAndPersonas,
            (false, true) => Self::SecurifyingAccount,
        }
    }

    pub fn pre_deriving_keys() -> Self {
        Self::PreDerivingKeys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DerivationPurpose;

    #[test]
    fn test_for_creating_account() {
        assert_eq!(
            SUT::creation_of_new_virtual_entity(CAP26EntityKind::Account),
            SUT::CreatingNewAccount
        )
    }

    #[test]
    fn test_for_creating_persona() {
        assert_eq!(
            SUT::creation_of_new_virtual_entity(CAP26EntityKind::Identity),
            SUT::CreatingNewPersona
        )
    }

    #[test]
    fn test_for_securifying_account_only() {
        assert_eq!(
            SUT::for_securifying_or_updating(&IndexSet::from_iter([
                AccountAddress::sample().into()
            ])),
            SUT::SecurifyingAccount
        )
    }

    #[test]
    fn test_for_securifying_persona_only() {
        assert_eq!(
            SUT::for_securifying_or_updating(&IndexSet::from_iter([
                IdentityAddress::sample().into()
            ])),
            SUT::SecurifyingPersona
        )
    }

    #[test]
    fn test_for_securifying_account_and_persona() {
        assert_eq!(
            SUT::for_securifying_or_updating(&IndexSet::from_iter([
                AccountAddress::sample().into(),
                IdentityAddress::sample().into()
            ])),
            SUT::SecurifyingAccountsAndPersonas
        )
    }

    #[test]
    fn test_for_pre_deriving_keys() {
        assert_eq!(SUT::pre_deriving_keys(), SUT::PreDerivingKeys)
    }
}
