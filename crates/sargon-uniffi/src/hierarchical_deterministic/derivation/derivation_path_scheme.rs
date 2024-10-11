use crate::prelude::*;
use sargon::DerivationPathScheme as InternalDerivationPathScheme;

/// Which derivation path to used for some particular HD operations
/// such as signing or public key derivation. Radix Babylon introduces
/// a new scheme call Cap26 but we also need to support BIP44-like used
/// by Olympia.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum DerivationPathScheme {
    /// A BIP32 based derivation path scheme, using SLIP10.
    Cap26,

    /// A BIP32 based similar to BIP44, but not strict BIP44 since the
    /// last path component is hardened (a mistake made during Olympia),
    /// used to support legacy accounts imported from Olympia wallet.
    Bip44Olympia,
}
