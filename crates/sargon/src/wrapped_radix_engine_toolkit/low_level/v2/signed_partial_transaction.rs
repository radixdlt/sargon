use crate::prelude::*;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct SignedPartialTransaction {
    pub partial_transaction: PartialTransaction,
    pub root_subintent_signatures: IntentSignaturesV2,
    pub non_root_subintent_signatures: NonRootSubintentSignatures,
}

impl Serialize for SignedPartialTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = RET_compile_signed_partial_tx(&self.clone().into())
            .map_err(|_| {
                serde::ser::Error::custom(
                    CommonError::InvalidSignedPartialTransactionFailedToCompile,
                )
            })?;
        let bag_of_bytes: BagOfBytes = bytes.into();
        serializer.serialize_str(&bag_of_bytes.to_hex())
    }
}

impl<'de> Deserialize<'de> for SignedPartialTransaction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SignedPartialTransactionVisitor;

        impl<'de> Visitor<'de> for SignedPartialTransactionVisitor {
            type Value = SignedPartialTransaction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a byte array representing a SignedPartialTransaction",
                )
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                RET_decompile_signed_partial_tx(v)
                    .map_err(|_| {
                        de::Error::custom(CommonError::InvalidSignedPartialTransactionFailedToDecompile)
                    })
                    .and_then(|scrypto| {
                        SignedPartialTransaction::try_from(scrypto)
                            .map_err(de::Error::custom)
                    })
            }
        }

        deserializer.deserialize_bytes(SignedPartialTransactionVisitor)
    }
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
        let non_root_subintents: NonRootSubintents =
            value.partial_transaction.non_root_subintents.try_into()?;
        let partial_transaction = PartialTransaction::new(
            root_subintent.clone(),
            non_root_subintents,
        );
        let root_subintent_signatures: IntentSignaturesV2 = (
            value.root_subintent_signatures,
            root_subintent.transaction_intent_hash().hash,
        )
            .try_into()?;
        let non_root_subintent_signatures: NonRootSubintentSignatures = (
            value.non_root_subintent_signatures,
            root_subintent.transaction_intent_hash().hash,
        )
            .try_into()?;
        Ok(Self {
            partial_transaction,
            root_subintent_signatures,
            non_root_subintent_signatures,
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
