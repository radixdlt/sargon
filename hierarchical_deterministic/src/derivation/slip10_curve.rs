use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub enum SLIP10Curve {
    /// Curve25519 or Ed25519
    Curve25519,

    /// The bitcoin curve, used by Radix Olympia and still valid
    /// to support legacy accounts.
    Secp256k1,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use wallet_kit_common::json::{
        assert_json_roundtrip, assert_json_value_eq_after_roundtrip,
        assert_json_value_ne_after_roundtrip,
    };

    use super::SLIP10Curve;

    #[test]
    fn json_roundtrip_secp256k1() {
        let model = SLIP10Curve::Secp256k1;
        assert_json_value_eq_after_roundtrip(&model, json!("secp256k1"));
        assert_json_value_ne_after_roundtrip(&model, json!("curve25519"));
        assert_json_roundtrip(&model);
    }
    #[test]
    fn json_roundtrip_curve25519() {
        let model = SLIP10Curve::Curve25519;
        assert_json_value_eq_after_roundtrip(&model, json!("curve25519"));
        assert_json_value_ne_after_roundtrip(&model, json!("secp256k1"));
        assert_json_roundtrip(&model);
    }
}
