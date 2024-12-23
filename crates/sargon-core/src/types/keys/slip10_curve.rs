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

impl Display for SLIP10Curve {
    #[cfg(not(tarpaulin_include))] // false negative
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id())
    }
}

impl FromStr for SLIP10Curve {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Self::Curve25519.id() {
            Ok(Self::Curve25519)
        } else if s == Self::Secp256k1.id() {
            Ok(Self::Secp256k1)
        } else {
            Err(CommonError::UnknownSLIP10Curve {
                bad_value: s.to_string(),
            })
        }
    }
}

impl HasSampleValues for SLIP10Curve {
    fn sample() -> Self {
        Self::Curve25519
    }

    fn sample_other() -> Self {
        Self::Secp256k1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SLIP10Curve;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn json_roundtrip_secp256k1() {
        let model = SUT::Secp256k1;
        assert_json_value_eq_after_roundtrip(&model, json!("secp256k1"));
        assert_json_value_ne_after_roundtrip(&model, json!("curve25519"));
        assert_json_roundtrip(&model);
    }

    #[test]
    fn json_roundtrip_curve25519() {
        let model = SUT::Curve25519;
        assert_json_value_eq_after_roundtrip(&model, json!("curve25519"));
        assert_json_value_ne_after_roundtrip(&model, json!("secp256k1"));
        assert_json_roundtrip(&model);
    }

    #[test]
    fn id() {
        assert_eq!(SUT::Curve25519.id(), "curve25519");
        assert_eq!(SUT::Secp256k1.id(), "secp256k1");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(SUT::Curve25519, SUT::from_str("curve25519").unwrap());
        assert_eq!(SUT::Secp256k1, SUT::from_str("secp256k1").unwrap());
    }

    #[test]
    fn test_from_str_bad_value() {
        assert_eq!(
            Err(CommonError::UnknownSLIP10Curve {
                bad_value: "bad value".to_string()
            }),
            SUT::from_str("bad value")
        );
    }
}
