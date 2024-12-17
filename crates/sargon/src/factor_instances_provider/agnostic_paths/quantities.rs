use crate::prelude::*;

/// The quantity of DerivationPreset's to fill cache with.
pub const CACHE_FILLING_QUANTITY: usize = 30;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::AccountVeci`.
pub const CACHE_FILLING_QUANTITY_ACCOUNT_VECI: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::AccountMfa`.
pub const CACHE_FILLING_QUANTITY_ACCOUNT_MFA: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::PersonaVeci`.
pub const CACHE_FILLING_QUANTITY_PERSONA_VECI: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::PersonaMfa`.
pub const CACHE_FILLING_QUANTITY_PERSONA_MFA: usize = CACHE_FILLING_QUANTITY;

impl DerivationPreset {
    /// The quantity of DerivationPreset's to fill cache with.
    pub fn cache_filling_quantity(&self) -> usize {
        match self {
            Self::AccountVeci => CACHE_FILLING_QUANTITY_ACCOUNT_VECI,
            Self::AccountMfa => CACHE_FILLING_QUANTITY_ACCOUNT_MFA,
            Self::PersonaVeci => CACHE_FILLING_QUANTITY_PERSONA_VECI,
            Self::PersonaMfa => CACHE_FILLING_QUANTITY_PERSONA_MFA,
        }
    }
}
