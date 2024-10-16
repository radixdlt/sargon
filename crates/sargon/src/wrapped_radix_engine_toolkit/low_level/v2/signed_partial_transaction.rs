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
                    "a hex string representing a SignedPartialTransaction",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let bytes = BagOfBytes::from_hex(v)
                    .map_err(|e| de::Error::custom(e))?;

                RET_decompile_signed_partial_tx(bytes.to_vec())
                    .map_err(|_| {
                        de::Error::custom(CommonError::InvalidSignedPartialTransactionFailedToDecompile)
                    })
                    .and_then(|scrypto| {
                        SignedPartialTransaction::try_from(scrypto)
                            .map_err(de::Error::custom)
                    })
            }
        }

        deserializer.deserialize_str(SignedPartialTransactionVisitor)
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
    use serde::de::value::{
        Error as ValueError, StrDeserializer, U64Deserializer,
    };
    use serde::de::IntoDeserializer;

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

    #[test]
    fn serialize() {
        let sut = SUT::sample();
        let encoded_hex = "4d220e03210221012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202001072048f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a9352022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f727421028100000000220000202101012105210607010a872c0100000000000a912c01000000000022010105008306670000000022010105e8860667000000000a15cd5b070000000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202001072048f1bd08444b5e713db9e14caac2faae71836786ac94d645b00679728202a9352022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f7274210281000000002200002022030001210120074100bef5d7e49752f0fe86043bc9f09b76837427f2fd50711a59d05748dd4fa32fed2c93677fee1b512a391bffc2bb137d47b908fef9420257ba4cd421d83d9baddc00012101200741009885b8bc32817b5005b612e6441506e8130c306b22ab86dba80d187732ae42c1212579ca78d4f355c9f00d24729aad3a4fe933212670b40e7d2d68176e87c2130001210120074101635ab6c2ca718616a872f5767ff3aaf1457d2580058818c215d905e8692fa9c97766e76e515ca5998572fa4c395302c4dd92b165ae41c28d2c8333e93556271820200122030001210120074100bef5d7e49752f0fe86043bc9f09b76837427f2fd50711a59d05748dd4fa32fed2c93677fee1b512a391bffc2bb137d47b908fef9420257ba4cd421d83d9baddc00012101200741009885b8bc32817b5005b612e6441506e8130c306b22ab86dba80d187732ae42c1212579ca78d4f355c9f00d24729aad3a4fe933212670b40e7d2d68176e87c2130001210120074101635ab6c2ca718616a872f5767ff3aaf1457d2580058818c215d905e8692fa9c97766e76e515ca5998572fa4c395302c4dd92b165ae41c28d2c8333e935562718".to_string();
        let serialized = serde_json::to_value(sut.clone()).unwrap();

        assert_eq!(serialized, encoded_hex)
    }
}
