use crate::prelude::*;

#[derive(Debug, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct WalletToDappInteractionAuthProof {
    pub public_key: String,
    pub curve: SLIP10Curve,
    pub signature: String,
}

impl HasSampleValues for WalletToDappInteractionAuthProof {
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
