use std::fmt::Display;

use crate::prelude::*;

/// Elliptic Curves which the SLIP10 derivation algorithm supports.
///
/// We use SLIP10 for hierarchical deterministic derivation since we
/// prefer using Curve25519 - which is incompatible with BIP32 (BIP44).
///
/// For for information see [SLIP10 reference](https://github.com/satoshilabs/slips/blob/master/slip-0010.md)
#[derive(
    Serialize,
    Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    uniffi::Enum,
)]
#[serde(rename_all = "camelCase")]
pub enum SLIP10Curve {
    /// Curve25519 which we use for Ed25519 for EdDSA signatures.
    Curve25519,

    /// The bitcoin curve, used by Radix Olympia and still valid
    /// to support legacy accounts.
    Secp256k1,
}

impl Identifiable for SLIP10Curve {
    type ID = String;

    fn id(&self) -> Self::ID {
        match self {
            Self::Curve25519 => "curve25519".to_string(),
            Self::Secp256k1 => "secp256k1".to_string(),
        }
    }
}

json_string_convertible!(SLIP10Curve, "super invalid json string");

#[uniffi::export]
pub fn new_slip10_curve_from_string(curve: String) -> Result<SLIP10Curve> {
    SLIP10Curve::from_str(&curve)
}

#[uniffi::export]
pub fn slip10_curve_to_string(curve: SLIP10Curve) -> String {
    curve.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_roundtrip() {
        assert_eq!(
            new_slip10_curve_from_string(slip10_curve_to_string(
                SLIP10Curve::Curve25519
            ))
            .unwrap(),
            SLIP10Curve::Curve25519
        )
    }
}