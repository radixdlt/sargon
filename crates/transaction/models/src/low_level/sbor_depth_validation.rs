#[cfg(test)]
use radix_common::prelude::{
    manifest_encode as Scrypto_manifest_encode,
    ScryptoValue as ScryptoScryptoValue, MANIFEST_SBOR_V1_MAX_DEPTH,
    SCRYPTO_SBOR_V1_MAX_DEPTH,
};
#[cfg(test)]
use sbor::{
    CustomValue as ScryptoCustomValue,
    CustomValueKind as ScryptoCustomValueKind, Value as ScryptoValue,
};

#[cfg(test)]
fn sbor_value_with_depth<X, Y>(depth: usize) -> ScryptoValue<X, Y>
where
    X: ScryptoCustomValueKind,
    Y: ScryptoCustomValue<X>,
{
    let mut value = sbor::Value::Tuple { fields: vec![] };
    for _ in 0..depth - 1 {
        value = sbor::Value::Tuple {
            fields: vec![value],
        }
    }
    value
}

#[cfg(test)]
pub(crate) fn scrypto_value_with_sbor_depth(
    depth: usize,
) -> ScryptoScryptoValue {
    sbor_value_with_depth(depth)
}

#[cfg(test)]
use radix_common::prelude::ManifestValue as ScryptoManifestValue;

#[cfg(test)]
pub(crate) fn manifest_value_with_sbor_depth(
    depth: usize,
) -> ScryptoManifestValue {
    sbor_value_with_depth(depth)
}

#[cfg(test)]
mod sbor_depth_validation_tests {
    use addresses::Scrypto_scrypto_encode;

    use super::*;

    #[test]
    fn scrypto_value_at_max_depth_is_encodable() {
        let value = scrypto_value_with_sbor_depth(SCRYPTO_SBOR_V1_MAX_DEPTH);
        Scrypto_scrypto_encode(&value).unwrap();
    }

    #[test]
    #[should_panic]
    fn scrypto_value_exceeding_max_depth_is_not_encodable() {
        let value =
            scrypto_value_with_sbor_depth(SCRYPTO_SBOR_V1_MAX_DEPTH + 1);
        Scrypto_scrypto_encode(&value).unwrap();
    }

    #[test]
    fn manifest_value_at_max_depth_is_encodable() {
        let value = manifest_value_with_sbor_depth(MANIFEST_SBOR_V1_MAX_DEPTH);
        Scrypto_manifest_encode(&value).unwrap();
    }

    #[test]
    #[should_panic]
    fn manifest_value_exceeding_max_depth_is_not_encodable() {
        let value =
            manifest_value_with_sbor_depth(MANIFEST_SBOR_V1_MAX_DEPTH + 1);
        Scrypto_manifest_encode(&value).unwrap();
    }
}
