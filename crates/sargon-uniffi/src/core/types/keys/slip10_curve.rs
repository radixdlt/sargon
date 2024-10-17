use crate::prelude::*;
use sargon::SLIP10Curve as InternalSLIP10Curve;

/// Elliptic Curves which the SLIP10 derivation algorithm supports.
///
/// We use SLIP10 for hierarchical deterministic derivation since we
/// prefer using Curve25519 - which is incompatible with BIP32 (BIP44).
///
/// For for information see [SLIP10 reference](https://github.com/satoshilabs/slips/blob/master/slip-0010.md)
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum SLIP10Curve {
    /// Curve25519 which we use for Ed25519 for EdDSA signatures.
    Curve25519,

    /// The bitcoin curve, used by Radix Olympia and still valid
    /// to support legacy accounts.
    Secp256k1,
}

json_string_convertible!(SLIP10Curve);

#[uniffi::export]
pub fn new_slip10_curve_from_string(curve: String) -> Result<SLIP10Curve> {
    InternalSLIP10Curve::from_str(&curve).into_result()
}

#[uniffi::export]
pub fn slip10_curve_to_string(curve: SLIP10Curve) -> String {
    curve.into_internal().to_string()
}
