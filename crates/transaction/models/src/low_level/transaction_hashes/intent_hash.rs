use crate::prelude::*;

impl HasSampleValues for TransactionIntentHash {
    fn sample() -> Self {
        TransactionIntentHash::new(Hash::sample(), NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        intent.transaction_intent_hash()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionIntentHash;

    #[test]
    fn from_hash() {
        let hash: Hash =
            "60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"
                .parse()
                .unwrap();
        assert_eq!(SUT::new(hash, NetworkID::Simulator), SUT::sample_other())
    }

    #[test]
    fn into_hash() {
        assert_eq!(
            Hash::from(SUT::sample_other()),
            "60e5617d670e6c8a42ba5f3749f4ff1079f66221f282554ecdda9ad385ecb195"
                .parse::<Hash>()
                .unwrap()
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", SUT::sample()),
            SUT::sample().bech32_encoded_tx_id.to_string()
        );
        assert_eq!(format!("{}", SUT::sample()), "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd");
        assert_eq!(format!("{}", SUT::sample_other()), "txid_sim1vrjkzlt8pekg5s46tum5na8lzpulvc3p72p92nkdm2dd8p0vkx2svr7ejr");
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", SUT::sample()), "txid_rdx1frcm6zzyfd08z0deu9x24sh64eccxeux4j2dv3dsqeuh9qsz4y6szm3ltd");
    }
}
