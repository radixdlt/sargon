use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// A Transaction item that is part of the interaction queue.
pub struct TransactionQueueItem {
    /// The identifier of the transaction.
    pub transaction_id: TransactionIntentHash,

    /// Hex-encoded notarized transaction payload which can be submitted to network.
    pub notarized_transaction_hex: String,
}

impl TransactionQueueItem {
    pub fn new(
        transaction_id: TransactionIntentHash,
        notarized_transaction_hex: String,
    ) -> Self {
        Self {
            transaction_id,
            notarized_transaction_hex,
        }
    }
}

impl From<NotarizedTransaction> for TransactionQueueItem {
    fn from(value: NotarizedTransaction) -> Self {
        Self::new(
            value.signed_intent().intent().transaction_intent_hash(),
            value.compile().to_string(),
        )
    }
}

impl HasSampleValues for TransactionQueueItem {
    fn sample() -> Self {
        Self::new(
            TransactionIntentHash::sample(),
            "4d22030221022104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202200220101210120074065938bf04b155de7277d95582ef2f5d36f7200765ee730cf3658da1861ad6e5008df90ac53d2835a48a5c0cb58891297761bda9533411e7eeddb1557d5dbe30a".to_owned(),
        )
    }

    fn sample_other() -> Self {
        Self::new(
            TransactionIntentHash::sample_other(),
            "4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b77ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd88c14a27b68a123".to_owned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionQueueItem;

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
    fn from() {
        let notarized_transaction = NotarizedTransaction::sample();
        let sut = SUT::from(notarized_transaction.clone());
        assert_eq!(
            sut.transaction_id,
            notarized_transaction
                .signed_intent()
                .intent()
                .transaction_intent_hash()
        );
        assert_eq!(
            sut.notarized_transaction_hex,
            notarized_transaction.compile().to_string()
        );
    }
}
