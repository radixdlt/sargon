use crate::prelude::*;

decl_version_type!(Subintent);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSubintentRequestItem {
    pub version: SubintentVersion,

    #[serde(flatten, with = "UnvalidatedSubintentManifest")]
    pub unvalidated_manifest: UnvalidatedSubintentManifest,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<DappToWalletInteractionSubintentExpiration>,
}

impl DappToWalletInteractionSubintentRequestItem {
    pub fn new(
        version: impl Into<SubintentVersion>,
        unvalidated_manifest: impl Into<UnvalidatedSubintentManifest>,
        message: impl Into<Option<String>>,
        expiration: impl Into<Option<DappToWalletInteractionSubintentExpiration>>,
    ) -> Self {
        Self {
            version: version.into(),
            unvalidated_manifest: unvalidated_manifest.into(),
            message: message.into(),
            expiration: expiration.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionSubintentRequestItem {
    fn sample() -> Self {
        Self::new(
            SubintentVersion::sample(),
            UnvalidatedSubintentManifest::sample(),
            "message".to_owned(),
            DappToWalletInteractionSubintentExpiration::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SubintentVersion::sample_other(),
            UnvalidatedSubintentManifest::sample_other(),
            "message_other".to_owned(),
            DappToWalletInteractionSubintentExpiration::sample_other(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionSubintentRequestItem;

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
    fn json_roundtrip() {
        assert_eq_after_json_roundtrip(
            &SUT::sample(),
            r#"
           {
                "version" : 1,
                "subintentManifest" : "CALL_METHOD\n    Address(\"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr\")\n    \"lock_fee\"\n    Decimal(\"0.61\")\n;\nCALL_METHOD\n    Address(\"account_rdx128y6j78mt0aqv6372evz28hrxp8mn06ccddkr7xppc88hyvynvjdwr\")\n    \"withdraw\"\n    Address(\"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd\")\n    Decimal(\"1337\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd\")\n    Decimal(\"1337\")\n    Bucket(\"bucket1\")\n;\nCALL_METHOD\n    Address(\"account_rdx12xkzynhzgtpnnd02tudw2els2g9xl73yk54ppw8xekt2sdrlaer264\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket1\")\n    Enum<0u8>()\n;\n",
                "blobs" : [],
                "message" : "message",
                "expiration": {
                    "discriminator": "expireAtTime",
                    "unixTimestampSeconds": "2023-09-11T16:05:56.000Z"
                }
            }
            "#,
        );
    }
}
