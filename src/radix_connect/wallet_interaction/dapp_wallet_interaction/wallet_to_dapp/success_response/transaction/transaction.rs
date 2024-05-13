use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionTransactionResponseItems {
    pub send: WalletToDappInteractionSendTransactionResponseItem,
}

impl WalletToDappInteractionTransactionResponseItems {
    pub fn new(transaction_intent_hash: IntentHash) -> Self {
        Self {
            send: WalletToDappInteractionSendTransactionResponseItem::new(
                transaction_intent_hash,
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct WalletToDappInteractionSendTransactionResponseItem {
    #[serde(rename = "transactionIntentHash")]
    bech32_encoded_tx_id: String,
}

impl WalletToDappInteractionSendTransactionResponseItem {
    pub fn new(transaction_intent_hash: IntentHash) -> Self {
        Self {
            bech32_encoded_tx_id: transaction_intent_hash.bech32_encoded_tx_id,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionTransactionResponseItems {
    fn sample() -> Self {
        Self::new(IntentHash::sample())
    }
    fn sample_other() -> Self {
        Self::new(IntentHash::sample_other())
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
}
