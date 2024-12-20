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

    /// Returns a the QuantifiedDerivationPresets needed to securify the `addresses_of_entities`, including
    /// a new Authentication Signing factor instance for each entity. Will return
    /// the `Account` variant of each DerivationPreset for each Account in `addresses_of_entities`
    /// and the `Identity` variant of each DerivationPreset for each Persona in `addresses_of_entities`.
    pub fn securifying_unsecurified_entities(
        addresses_of_entities: &IndexSet<AddressOfAccountOrPersona>,
    ) -> IdentifiedVecOf<Self> {
        Self::mfa_for_entities(addresses_of_entities, true)
    }

    /// Returns a the QuantifiedDerivationPresets needed to securify the `addresses_of_entities`,  Will return
    /// the `Account` variant of each DerivationPreset for each Account in `addresses_of_entities`
    /// and the `Identity` variant of each DerivationPreset for each Persona in `addresses_of_entities`.
    ///
    /// if `include_rola_key_for_each_entity` is `true` a ROLA key for each entity will be included.
    /// Typically we only set `include_rola_key_for_each_entity` to `true` for securifying
    /// unsecurified entities. For already securified entities we might not
    /// need to change the ROLA key.
    fn mfa_for_entities(
        addresses_of_entities: &IndexSet<AddressOfAccountOrPersona>,
        include_rola_key_for_each_entity: bool,
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
            (true, false) => {
                let mut presets = IdentifiedVecOf::just(Self::new(
                    DerivationPreset::IdentityMfa,
                    identity_addresses.len(),
                ));
                if include_rola_key_for_each_entity {
                    presets.append(Self::new(
                        DerivationPreset::IdentityRola,
                        identity_addresses.len(),
                    ));
                }
                presets
            }
            (false, false) => {
                let mut presets = IdentifiedVecOf::from_iter([
                    Self::new(
                        DerivationPreset::AccountMfa,
                        account_addresses.len(),
                    ),
                    Self::new(
                        DerivationPreset::IdentityMfa,
                        identity_addresses.len(),
                    ),
                ]);
                if include_rola_key_for_each_entity {
                    presets.append(Self::new(
                        DerivationPreset::AccountRola,
                        account_addresses.len(),
                    ));
                    presets.append(Self::new(
                        DerivationPreset::IdentityRola,
                        identity_addresses.len(),
                    ));
                }
                presets
            }
            (false, true) => {
                let mut presets = IdentifiedVecOf::just(Self::new(
                    DerivationPreset::AccountMfa,
                    account_addresses.len(),
                ));
                if include_rola_key_for_each_entity {
                    presets.append(Self::new(
                        DerivationPreset::AccountRola,
                        account_addresses.len(),
                    ));
                }
                presets
            }
        }
    }
}
