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
        self.derivation_preset
    }
}

impl QuantifiedDerivationPreset {
    pub fn new(derivation_preset: DerivationPreset, quantity: usize) -> Self {
        Self {
            derivation_preset,
            quantity,
        }
    }

    /// Returns a the QuantifiedDerivationPresets needed apply a shield update
    /// to `addresses_of_entities_to_derive_tx_key_for`. Will return the `Account` variant of each
    /// DerivationPreset for each Account in `addresses_of_entities_to_derive_tx_key_for` and the
    /// `Identity` variant of each DerivationPreset for each Persona
    /// in `addresses_of_entities_to_derive_tx_key_for`.
    ///
    /// We will derive ` `addresses_of_entities_to_derive_rola_key_for.len()` many
    /// ROLA keys.
    pub fn apply_shield_to_entities(
        addresses_of_entities_to_derive_tx_key_for: &IndexSet<AddressOfAccountOrPersona>,
        addresses_of_entities_to_derive_rola_key_for: &IndexSet<
            AddressOfAccountOrPersona,
        >,
    ) -> IdentifiedVecOf<Self> {
        fn from_addresses_for_entity(
            addresses: &IndexSet<AddressOfAccountOrPersona>,
            make_preset: impl Fn(CAP26EntityKind) -> DerivationPreset,
        ) -> IdentifiedVecOf<QuantifiedDerivationPreset> {
            let account_addresses =
                addresses.iter().filter(|a| a.is_account()).collect_vec();

            let identity_addresses =
                addresses.iter().filter(|a| a.is_identity()).collect_vec();

            match (account_addresses.is_empty(), identity_addresses.is_empty())
            {
                (true, true) => IdentifiedVecOf::new(), // weird!
                (true, false) => {
                    IdentifiedVecOf::just(QuantifiedDerivationPreset::new(
                        make_preset(CAP26EntityKind::Identity),
                        identity_addresses.len(),
                    ))
                }
                (false, false) => IdentifiedVecOf::from_iter([
                    QuantifiedDerivationPreset::new(
                        make_preset(CAP26EntityKind::Account),
                        account_addresses.len(),
                    ),
                    QuantifiedDerivationPreset::new(
                        make_preset(CAP26EntityKind::Identity),
                        identity_addresses.len(),
                    ),
                ]),
                (false, true) => {
                    IdentifiedVecOf::just(QuantifiedDerivationPreset::new(
                        make_preset(CAP26EntityKind::Account),
                        account_addresses.len(),
                    ))
                }
            }
        }

        let mut transaction = from_addresses_for_entity(
            addresses_of_entities_to_derive_tx_key_for,
            DerivationPreset::mfa_entity_kind,
        );
        let authentication = from_addresses_for_entity(
            addresses_of_entities_to_derive_rola_key_for,
            DerivationPreset::rola_entity_kind,
        );

        transaction.extend(authentication);
        transaction
    }
}
