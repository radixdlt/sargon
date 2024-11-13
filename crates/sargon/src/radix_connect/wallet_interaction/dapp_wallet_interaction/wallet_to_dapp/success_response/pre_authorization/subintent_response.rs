use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WalletToDappInteractionSubintentResponseItem {
    /// A hex encoded signed partial transaction.
    #[serde(rename = "signedPartialTransaction")]
    pub encoded_signed_partial_transaction: String,
}

impl WalletToDappInteractionSubintentResponseItem {
    pub fn new(signed_subintent: SignedSubintent) -> Self {
        let bytes = signed_subintent.compiled();
        let encoded_signed_partial_transaction = hex_encode(&bytes);
        Self {
            encoded_signed_partial_transaction,
        }
    }
}

impl HasSampleValues for WalletToDappInteractionSubintentResponseItem {
    fn sample() -> Self {
        Self {
            encoded_signed_partial_transaction:
            "4d220e03210221012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212020002022054103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f727421028100000000220000600121002021002022030001210120074101069ff73da4b2c861c340558d0d7ee44bfb8f221f4ba7f8d74a9e9d82c1acd2f951afd718faddb24a11062a508ad6edf31c519f88973688eaf04fc0cd944bebdc00012101200741007a324555c61268211c4dae7c63a5f94f3be523d7b0b93426685c8767d74907fb348775142bb6e1ac6d236acf795b672b7c94114e4198caec995d86d1327d5c060001210120074100fb65235bc47969a6b4421b8495641e9bec403103df5fa4ed7a123df0dc97f1734822bc9e609e00aa13698ba3227a61a8aff23fcc0f188eed9f29da155aa5c894202000".to_owned(),
        }
    }

    fn sample_other() -> Self {
        Self {
            encoded_signed_partial_transaction:
            "4d220e03210221012105210607f20a00000000000000000a0a000000000000002200002200000ab168de3a00000000202000220000202000202200202100202201000121012007410001598e989470d125dafac276b95bb1ba21e2ee8e0beb0547599335f83b48a0a830cd6a956a54421039cef5fb7e492ebaa315f751a2dd5b74bd9cebbda997ec12202000".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = WalletToDappInteractionSubintentResponseItem;

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
    fn new() {
        let signed_subintent = SignedSubintent::sample();
        let sut = SUT::new(signed_subintent);
        assert_eq!(sut, SUT::sample());
    }
}
