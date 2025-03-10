use serde::ser::SerializeStruct;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

impl WalletToDappInteractionTransactionResponseItems {
    pub fn new(transaction_intent_hash: TransactionIntentHash) -> Self {
        Self {
            send: WalletToDappInteractionSendTransactionResponseItem {
                transaction_intent_hash,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    pub transaction_intent_hash: TransactionIntentHash,
}

impl Serialize for WalletToDappInteractionSendTransactionResponseItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct(
            "WalletToDappInteractionSendTransactionResponseItem",
            1,
        )?;
        state.serialize_field(
            "transactionIntentHash",
            &self.transaction_intent_hash.bech32_encoded_tx_id,
        )?;
        state.end()
    }
}

impl<'de> Deserialize<'de>
    for WalletToDappInteractionSendTransactionResponseItem
{
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        #[derive(Deserialize, Serialize)]
        struct Wrapper {
            #[serde(rename = "transactionIntentHash")]
            transaction_intent_hash: String,
        }
        let wrapped = Wrapper::deserialize(deserializer)?;
        TransactionIntentHash::from_bech32(&wrapped.transaction_intent_hash)
            .map_err(de::Error::custom)
            .map(|i| Self {
                transaction_intent_hash: i,
            })
    }
}

impl HasSampleValues for WalletToDappInteractionTransactionResponseItems {
    fn sample() -> Self {
        Self {
            send: WalletToDappInteractionSendTransactionResponseItem::sample(),
        }
    }
    fn sample_other() -> Self {
        Self {
            send:
                WalletToDappInteractionSendTransactionResponseItem::sample_other(
                ),
        }
    }
}

impl HasSampleValues for WalletToDappInteractionSendTransactionResponseItem {
    fn sample() -> Self {
        Self {
            transaction_intent_hash: TransactionIntentHash::sample(),
        }
    }
    fn sample_other() -> Self {
        Self {
            transaction_intent_hash: TransactionIntentHash::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionTransactionResponseItems;

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
        let sut = SUT::sample();
        let json = r#"
        {
            "send": {
                "transactionIntentHash" : "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd"
            }
        }
        "#;
        assert_eq_after_json_roundtrip(&sut, json);
    }
}
