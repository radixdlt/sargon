use crate::{address_union, prelude::*};

address_union!(
    /// A tagged union of addresses of either an Account or a Persona (IdentityAddress)
    enum AddressOfAccountOrPersona: account, identity
);

impl IsBaseEntityAddress for AddressOfAccountOrPersona {}

impl HasEntityKindObjectSafe for AddressOfAccountOrPersona {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        match self {
            Self::Account(a) => a.get_entity_kind(),
            Self::Identity(i) => i.get_entity_kind(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AddressOfAccountOrPersona;

    #[test]
    fn sample_values_count() {
        let mut set = HashSet::<SUT>::new();
        set.extend(SUT::sample_values_all());
        // Duplicates should be removed
        set.extend(SUT::sample_values_all());

        assert_eq!(set.len(), 8);
    }
}
