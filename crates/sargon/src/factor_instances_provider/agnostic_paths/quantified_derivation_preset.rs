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
}
