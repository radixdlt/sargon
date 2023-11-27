use serde::{Deserialize, Serialize};
use wallet_kit_common::types::hex_32bytes::Hex32Bytes;

use super::factor_source_kind::FactorSourceKind;

/// FactorSourceID from a hash.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FactorSourceIDFromHash {
    pub kind: FactorSourceKind,
    pub body: Hex32Bytes,
}

impl FactorSourceIDFromHash {
    pub fn new(kind: FactorSourceKind, body: Hex32Bytes) -> Self {
        Self { kind, body }
    }
}

impl FactorSourceIDFromHash {
    pub fn placeholder() -> Self {
        Self {
            kind: FactorSourceKind::Device,
            body: Hex32Bytes::placeholder(),
        }
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::FactorSourceIDFromHash;

    #[test]
    fn json_roundtrip() {
        let model = FactorSourceIDFromHash::placeholder();

        assert_eq_after_json_roundtrip(
            &model,
            r#"
            {
                "kind": "device",
                "body": "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
            }
            "#,
        );
    }
}
