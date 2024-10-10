use crate::prelude::*;
use sargon::DerivationPathScheme as InternalDerivationPathScheme;

/// Which derivation path to used for some particular HD operations
/// such as signing or public key derivation. Radix Babylon introduces
/// a new scheme call Cap26 but we also need to support BIP44-like used
/// by Olympia.
#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum DerivationPathScheme {
    /// A BIP32 based derivation path scheme, using SLIP10.
    Cap26,

    /// A BIP32 based similar to BIP44, but not strict BIP44 since the
    /// last path component is hardened (a mistake made during Olympia),
    /// used to support legacy accounts imported from Olympia wallet.
    Bip44Olympia,
}

impl From<InternalDerivationPathScheme> for DerivationPathScheme {
    fn from(value: InternalDerivationPathScheme) -> Self {
        match value {
            InternalDerivationPathScheme::Cap26 => Self::Cap26,
            InternalDerivationPathScheme::Bip44Olympia => Self::Bip44Olympia,
        }
    }
}

impl Into<InternalDerivationPathScheme> for DerivationPathScheme {
    fn into(self) -> InternalDerivationPathScheme {
        match self {
            Self::Cap26 => InternalDerivationPathScheme::Cap26,
            Self::Bip44Olympia => InternalDerivationPathScheme::Bip44Olympia,
        }
    }
}