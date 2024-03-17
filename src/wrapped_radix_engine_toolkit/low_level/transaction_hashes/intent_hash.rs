use crate::prelude::*;

impl HasSampleValues for IntentHash {
    fn sample() -> Self {
        IntentHash::new(Hash::sample(), NetworkID::Mainnet)
    }

    fn sample_other() -> Self {
        let intent = TransactionIntent::sample_other();
        intent.intent_hash()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = IntentHash;

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
}
