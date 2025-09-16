use crate::prelude::*;

/// The quantity of DerivationPreset's to fill cache with.
pub const CACHE_FILLING_QUANTITY: usize = 30;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::AccountVeci`.
pub const CACHE_FILLING_QUANTITY_ACCOUNT_VECI: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::AccountMfa`.
pub const CACHE_FILLING_QUANTITY_ACCOUNT_MFA: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::AccountRola`.
pub const CACHE_FILLING_QUANTITY_ACCOUNT_ROLA: usize = CACHE_FILLING_QUANTITY;
/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::IdentityVeci`.
pub const CACHE_FILLING_QUANTITY_IDENTITY_VECI: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::IdentityMfa`.
pub const CACHE_FILLING_QUANTITY_IDENTITY_MFA: usize = CACHE_FILLING_QUANTITY;

/// The quantity of DerivationPreset's to fill cache with for `DerivationPreset::IdentityRola`.
pub const CACHE_FILLING_QUANTITY_IDENTITY_ROLA: usize = CACHE_FILLING_QUANTITY;
impl DerivationPreset {
    /// The quantity of DerivationPreset's to fill cache with.
    pub fn cache_filling_quantity(&self) -> usize {
        match self {
            Self::AccountVeci => CACHE_FILLING_QUANTITY_ACCOUNT_VECI,
            Self::AccountMfa => CACHE_FILLING_QUANTITY_ACCOUNT_MFA,
            Self::AccountRola => CACHE_FILLING_QUANTITY_ACCOUNT_ROLA,
            Self::IdentityVeci => CACHE_FILLING_QUANTITY_IDENTITY_VECI,
            Self::IdentityMfa => CACHE_FILLING_QUANTITY_IDENTITY_MFA,
            Self::IdentityRola => CACHE_FILLING_QUANTITY_IDENTITY_ROLA,
        }
    }
}
