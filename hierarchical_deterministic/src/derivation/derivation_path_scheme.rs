use serde::{Deserialize, Serialize};

use super::slip10_curve::SLIP10Curve;

/// Which derivation path to used for some particular HD operations
/// such as signing or public key derivation. Radix Babylon introduces
/// a new scheme call Cap26 but we also need to support BIP44-like used
/// by Olympia.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum DerivationPathScheme {
    /// A BIP32 based derivation path scheme, using SLIP10.
    Cap26,

    /// A BIP32 based similar to BIP44, but not strict BIP44 since the
    /// last path component is hardened (a mistake made during Olympia),
    /// used to support legacy accounts imported from Olympia wallet.
    Bip44Olympia,
}

impl DerivationPathScheme {
    /// The curve used for each derivation path scheme.
    ///
    /// We always use `curve25519` for non Olympia factor instances,
    /// given that the scheme is `cap26` it means it is a non Olympia factor
    /// instance => thus OK to always use `curve25519`
    ///  
    /// Bip44 is only used with `secp256k1` and `secp256k1` is only used for `bip44`
    /// scheme, thus OK to return `secp256k1`.
    pub fn curve(&self) -> SLIP10Curve {
        match self {
            Self::Cap26 => SLIP10Curve::Curve25519,
            Self::Bip44Olympia => SLIP10Curve::Secp256k1,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use crate::derivation::{
        derivation_path_scheme::DerivationPathScheme, slip10_curve::SLIP10Curve,
    };

    #[test]
    fn curve_from_scheme() {
        assert_eq!(DerivationPathScheme::Cap26.curve(), SLIP10Curve::Curve25519);
        assert_eq!(
            DerivationPathScheme::Bip44Olympia.curve(),
            SLIP10Curve::Secp256k1
        );
    }

    #[test]
    fn json_roundtrip_bip44() {
        let model = DerivationPathScheme::Bip44Olympia;
        assert_json_value_eq_after_roundtrip(&model, json!("bip44Olympia"));
        assert_json_value_ne_after_roundtrip(&model, json!("cap26"));
        assert_json_roundtrip(&model);
    }
    #[test]
    fn json_roundtrip_cap26() {
        let model = DerivationPathScheme::Cap26;
        assert_json_value_eq_after_roundtrip(&model, json!("cap26"));
        assert_json_value_ne_after_roundtrip(&model, json!("bip44Olympia"));
        assert_json_roundtrip(&model);
    }
}
