use crate::prelude::*;

use radix_engine_toolkit::functions::notarized_transaction::{
    compile as RET_compile_notarized_tx, decompile as RET_decompile_notarize_tx,
};

use transaction::model::{
    NotarizedTransactionV1 as ScryptoNotarizedTransaction,
    SignedIntentV1 as ScryptoSignedIntent,
};

#[derive(
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
    uniffi::Record,
)]
pub struct CompiledNotarizedIntent {
    pub(crate) secret_magic: BagOfBytes,
}

impl CompiledNotarizedIntent {
    pub fn new(bytes: BagOfBytes) -> Self {
        Self {
            secret_magic: bytes,
        }
    }

    pub fn decompile(&self) -> Result<NotarizedTransaction> {
        RET_decompile_notarize_tx(self.secret_magic.bytes())
        .map_err(|e| {
            error!("Failed to decompile bytes into Notarized Transaction, error: {:?}", e);
            CommonError::FailedToDecompileBytesIntoNotarizedTransaction
        })
        .and_then(TryInto::<NotarizedTransaction>::try_into)
    }
}

impl TryFrom<ScryptoNotarizedTransaction> for NotarizedTransaction {
    type Error = crate::CommonError;

    fn try_from(
        value: ScryptoNotarizedTransaction,
    ) -> Result<Self, Self::Error> {
        let signed_intent: SignedIntent = value.signed_intent.try_into()?;
        Ok(Self {
            signed_intent,
            notary_signature: value.notary_signature.into(),
        })
    }
}

impl HasSampleValues for CompiledNotarizedIntent {
    fn sample() -> Self {
        let bytes: BagOfBytes = "4d22030221022104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf010108000020220441038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c086c6f636b5f6665652101850000fda0c42777080000000000000000000000000000000041038000d1be9c042f627d98a01383987916d43cf439631ca1d8c8076d6754ab263d0c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a4800000000000000000000000000000041038000d1127918c16af09af521951adcf3a20ab2cc87c0e72e85814764853ce5e70c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f205261646978212022002201012101200740839ac9c47db45950fc0cd453c5ebbbfa7ae5f7c20753abe2370b5b40fdee89e522c4d810d060e0c56211d036043fd32b9908e97bf114c1835ca02d74018fdd09".parse().unwrap();

        Self {
            secret_magic: bytes,
        }
    }

    fn sample_other() -> Self {
        let bytes: BagOfBytes = "4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b77ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd88c14a27b68a123".parse().unwrap();

        Self {
            secret_magic: bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = CompiledNotarizedIntent;

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
    fn from_str() {
        assert_eq!("4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b77ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd88c14a27b68a123".parse::<SUT>().unwrap(), SUT::sample_other());
    }

    #[test]
    fn to_string() {
        assert_eq!(SUT::sample_other().to_string(), "4d22030221022104210707f20a00000000000000000a0a00000000000000090a0000002200012007210279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f817980101080000202200202000220000202200220001210120074101ebfc1f10a3b6ed83531f16249477ab86b77ce85980ef330abafbbd758caa98c665f68b8536112b6d1519feddeea01fd8429124dd75121d4bd88c14a27b68a123");
    }

    #[test]
    fn decompile() {
        assert_eq!(
            SUT::sample().decompile(),
            Ok(NotarizedTransaction::sample())
        );
        assert_eq!(
            SUT::sample_other().decompile(),
            Ok(NotarizedTransaction::sample_other())
        );
    }

    #[test]
    fn decompile_fail() {
        assert_eq!(
            SUT {
                secret_magic: BagOfBytes::sample_aced()
            }
            .decompile(),
            Err(CommonError::FailedToDecompileBytesIntoNotarizedTransaction)
        );
    }
}
