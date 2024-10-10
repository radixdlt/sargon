use crate::prelude::*;
use sargon::SLIP10Curve as InternalSLIP10Curve;

/// Elliptic Curves which the SLIP10 derivation algorithm supports.
///
/// We use SLIP10 for hierarchical deterministic derivation since we
/// prefer using Curve25519 - which is incompatible with BIP32 (BIP44).
///
/// For for information see [SLIP10 reference](https://github.com/satoshilabs/slips/blob/master/slip-0010.md)
#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum SLIP10Curve {
    /// Curve25519 which we use for Ed25519 for EdDSA signatures.
    Curve25519,

    /// The bitcoin curve, used by Radix Olympia and still valid
    /// to support legacy accounts.
    Secp256k1,
}

impl From<InternalSLIP10Curve> for SLIP10Curve {
    fn from(value: InternalSLIP10Curve) -> Self {
        match value {
            InternalSLIP10Curve::Curve25519 => SLIP10Curve::Curve25519,
            InternalSLIP10Curve::Secp256k1 => SLIP10Curve::Secp256k1,
        }
    }
}

impl Into<InternalSLIP10Curve> for SLIP10Curve {
    fn into(self) -> InternalSLIP10Curve {
        match self {
            SLIP10Curve::Curve25519 => InternalSLIP10Curve::Curve25519,
            SLIP10Curve::Secp256k1 => InternalSLIP10Curve::Secp256k1,
        }
    }
}

json_string_convertible!(SLIP10Curve, "super invalid json string");

#[uniffi::export]
pub fn new_slip10_curve_from_string(curve: String) -> Result<SLIP10Curve> {
    InternalSLIP10Curve::from_str(&curve).map_result()
}

#[uniffi::export]
pub fn slip10_curve_to_string(curve: SLIP10Curve) -> String {
    curve.into_internal().to_string()
}