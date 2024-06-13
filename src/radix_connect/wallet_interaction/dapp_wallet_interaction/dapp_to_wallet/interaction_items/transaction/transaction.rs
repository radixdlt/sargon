use crate::prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
pub struct DappToWalletInteractionTransactionItems {
    pub send: DappToWalletInteractionSendTransactionItem,
}

impl DappToWalletInteractionTransactionItems {
    pub fn new(send: DappToWalletInteractionSendTransactionItem) -> Self {
        Self { send }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, uniffi::Record)]
#[serde(rename_all = "camelCase")]
pub struct DappToWalletInteractionSendTransactionItem {
    #[serde(flatten, with = "UnvalidatedTransactionManifest")]
    pub unvalidated_manifest: UnvalidatedTransactionManifest,

    pub version: TXVersion,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl DappToWalletInteractionSendTransactionItem {
    pub fn new(
        unvalidated_manifest: impl Into<UnvalidatedTransactionManifest>,
        version: impl Into<TXVersion>,
        message: impl Into<Option<String>>,
    ) -> Self {
        Self {
            unvalidated_manifest: unvalidated_manifest.into(),
            version: version.into(),
            message: message.into(),
        }
    }
}

impl HasSampleValues for DappToWalletInteractionTransactionItems {
    fn sample() -> Self {
        Self::new(DappToWalletInteractionSendTransactionItem::sample())
    }

    fn sample_other() -> Self {
        Self::new(DappToWalletInteractionSendTransactionItem::sample_other())
    }
}

impl HasSampleValues for DappToWalletInteractionSendTransactionItem {
    fn sample() -> Self {
        Self::new(
            UnvalidatedTransactionManifest::sample(),
            TXVersion::sample(),
            "message".to_owned(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            UnvalidatedTransactionManifest::sample_other(),
            TXVersion::sample_other(),
            "message_other".to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DappToWalletInteractionTransactionItems;

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
        let json = r#"
        {
            "send" : {
                "version" : 2,
                "message" : "message_other",
                "blobs" : [
                "deadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafedeadbeefabbafadecafe"
                ],
                "transactionManifest" : "CALL_METHOD\n    Address(\"account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q\")\n    \"lock_fee\"\n    Decimal(\"500\")\n;\nCALL_METHOD\n    Address(\"account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q\")\n    \"withdraw\"\n    Address(\"resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3\")\n    Decimal(\"330\")\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3\")\n    Decimal(\"150\")\n    Bucket(\"bucket1\")\n;\nCALL_METHOD\n    Address(\"account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket1\")\n    Enum<0u8>()\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3\")\n    Decimal(\"130\")\n    Bucket(\"bucket2\")\n;\nCALL_METHOD\n    Address(\"account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket2\")\n    Enum<0u8>()\n;\nTAKE_FROM_WORKTOP\n    Address(\"resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3\")\n    Decimal(\"50\")\n    Bucket(\"bucket3\")\n;\nCALL_METHOD\n    Address(\"account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva\")\n    \"try_deposit_or_abort\"\n    Bucket(\"bucket3\")\n    Enum<0u8>()\n;\n"
            }
        }
        "#;
        let sut = SUT::sample_other();
        let deserialized: SUT = serde_json::from_str(json).unwrap();
        pretty_assertions::assert_eq!(deserialized, sut);
        assert_json_roundtrip(&sut);
    }
}
