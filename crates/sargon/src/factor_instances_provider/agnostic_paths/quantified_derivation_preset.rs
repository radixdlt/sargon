use crate::prelude::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, derive_more::Debug)]
#[debug("🎯: {:?} #{}", self.derivation_preset, self.quantity)]
pub struct QuantifiedDerivationPreset {
    pub derivation_preset: DerivationPreset,
    pub quantity: usize,
}

impl Identifiable for QuantifiedDerivationPreset {
    type ID = DerivationPreset;
    fn id(&self) -> DerivationPreset {
        self.derivation_preset.clone()
    }
}

impl QuantifiedDerivationPreset {
    pub fn new(derivation_preset: DerivationPreset, quantity: usize) -> Self {
        Self {
            derivation_preset,
            quantity,
        }
    }

    pub fn mfa_for_entities(
        addresses_of_entities: &IndexSet<AddressOfAccountOrPersona>,
    ) -> IdentifiedVecOf<Self> {
        let account_addresses = addresses_of_entities
            .iter()
            .filter(|a| a.is_account())
            .collect_vec();
        let identity_addresses = addresses_of_entities
            .iter()
            .filter(|a| a.is_identity())
            .collect_vec();

        match (account_addresses.is_empty(), identity_addresses.is_empty()) {
            (true, true) => IdentifiedVecOf::new(), // weird!
            (true, false) => IdentifiedVecOf::just(Self::new(
                DerivationPreset::IdentityMfa,
                identity_addresses.len(),
            )),
            (false, false) => IdentifiedVecOf::from_iter([
                Self::new(
                    DerivationPreset::AccountMfa,
                    account_addresses.len(),
                ),
                Self::new(
                    DerivationPreset::IdentityMfa,
                    identity_addresses.len(),
                ),
            ]),
            (false, true) => IdentifiedVecOf::just(Self::new(
                DerivationPreset::AccountMfa,
                account_addresses.len(),
            )),
        }
    }
}
