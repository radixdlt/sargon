use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct SignedPartialTransaction {
    pub partial_transaction: PartialTransaction,
    pub root_subintent_signatures: IntentSignaturesV2,
    pub non_root_subintent_signatures: NonRootSubintentSignatures,
}

impl SignedPartialTransaction {
    pub fn new(
        partial_transaction: PartialTransaction,
        root_subintent_signatures: IntentSignaturesV2,
        non_root_subintent_signatures: NonRootSubintentSignatures,
    ) -> Self {
        Self {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
        }
    }
}

impl From<SignedPartialTransaction> for ScryptoSignedPartialTransaction {
    fn from(value: SignedPartialTransaction) -> Self {
        Self {
            partial_transaction: value.partial_transaction.into(),
            root_subintent_signatures: value.root_subintent_signatures.into(),
            non_root_subintent_signatures: value
                .non_root_subintent_signatures
                .into(),
        }
    }
}

impl TryFrom<ScryptoSignedPartialTransaction> for SignedPartialTransaction {
    type Error = crate::CommonError;

    fn try_from(value: ScryptoSignedPartialTransaction) -> Result<Self> {
        let root_subintent: Subintent =
            value.partial_transaction.root_subintent.try_into()?;
        let partial_transaction =
            PartialTransaction::with_root_subintent(root_subintent.clone());
        let root_subintent_signatures: IntentSignaturesV2 = (
            value.root_subintent_signatures,
            root_subintent.transaction_intent_hash().hash,
        )
            .try_into()?;
        Ok(Self {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures: Default::default(),
        })
    }
}

impl HasSampleValues for SignedPartialTransaction {
    fn sample() -> Self {
        Self {
            partial_transaction: PartialTransaction::sample(),
            root_subintent_signatures: IntentSignaturesV2::sample(),
            non_root_subintent_signatures: NonRootSubintentSignatures::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            partial_transaction: PartialTransaction::sample_other(),
            root_subintent_signatures: IntentSignaturesV2::sample_other(),
            non_root_subintent_signatures:
                NonRootSubintentSignatures::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedPartialTransaction;

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
    fn to_from_scrypto() {
        let roundtrip = |s: SUT| {
            SUT::try_from(ScryptoSignedPartialTransaction::from(s)).unwrap()
        };
        roundtrip(SUT::sample());
        roundtrip(SUT::sample_other());
    }
}
