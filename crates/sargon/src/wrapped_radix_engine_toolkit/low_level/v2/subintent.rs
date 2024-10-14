use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug, uniffi::Record)]
pub struct Subintent {
    pub intent_core: IntentCoreV2,
}

impl Subintent {
    pub fn transaction_intent_hash(&self) -> TransactionIntentHash {
        let hash = ret_hash_subintent(&ScryptoSubintent::from(self.clone()))
            .expect("Should never fail to hash an subintent. Sargon should only produce valid Subintents");

        TransactionIntentHash::from_scrypto(
            ScryptoTransactionIntentHash(hash.hash),
            self.intent_core.header.network_id,
        )
    }
}

impl From<Subintent> for ScryptoSubintent {
    fn from(value: Subintent) -> Self {
        Self {
            intent_core: value.intent_core.into(),
        }
    }
}

impl TryFrom<ScryptoSubintent> for Subintent {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoSubintent) -> Result<Self> {
        Ok(Self {
            intent_core: value.intent_core.try_into()?,
        })
    }
}

impl HasSampleValues for Subintent {
    fn sample() -> Self {
        Self {
            intent_core: IntentCoreV2::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            intent_core: IntentCoreV2::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Subintent;

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
    fn transaction_intent_hash() {
        let subintent = SUT::sample();
        let hash = subintent.transaction_intent_hash();
        assert_eq!(hash.to_string(), "txid_rdx1gelylz5h59uk4enfnxe9vyaq69vurpt67y94uduehh3y8y2e3xfsddsz03".to_string());
    }

    #[test]
    fn to_from_scrypto() {
        let roundtrip =
            |s: SUT| SUT::try_from(ScryptoSubintent::from(s)).unwrap();
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
