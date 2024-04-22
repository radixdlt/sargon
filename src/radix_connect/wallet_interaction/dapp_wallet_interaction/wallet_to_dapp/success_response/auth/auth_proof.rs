use crate::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappWalletInteractionAuthProof {
    pub public_key: String,
    pub curve: SLIP10Curve,
    pub signature: String,
}

impl HasSampleValues for DappWalletInteractionAuthProof {
    fn sample() -> Self {
        Self {
            public_key: "sample1".to_string(),
            curve: SLIP10Curve::sample(),
            signature: "sample2".to_string(),
        }
    }

    fn sample_other() -> Self {
        Self {
            public_key: "sample3".to_string(),
            curve: SLIP10Curve::sample_other(),
            signature: "sample4".to_string(),
        }
    }
}
