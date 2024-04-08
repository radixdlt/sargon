use crate::prelude::*;

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
    secret_magic: BagOfBytes,
}

impl CompiledNotarizedIntent {
    pub fn new(bytes: BagOfBytes) -> Self {
        Self {
            secret_magic: bytes,
        }
    }

    pub fn bytes(&self) -> BagOfBytes {
        self.secret_magic.clone()
    }

    pub fn decompile(&self) -> NotarizedTransaction {
        let err = "Should never fail to decompile a 'CompiledNotarizedIntent' since we should not have been able to construct an invalid 'CompiledNotarizedIntent.";

        let notarized =
            RET_decompile_notarize_tx(self.secret_magic.bytes()).expect(err);

        notarized.try_into().expect(err)
    }
}

pub(crate) fn compile_notarized_intent(
    scrypto_notarized_intent: ScryptoNotarizedTransaction,
) -> Result<CompiledNotarizedIntent> {
    RET_compile_notarized_tx(&scrypto_notarized_intent)
        .map_err(|e| match e {
            sbor::EncodeError::MaxDepthExceeded(max) => {
                CommonError::InvalidTransactionMaxSBORDepthExceeded {
                    max: max as u16,
                }
            }
            _ => CommonError::InvalidNotarizedIntentFailedToEncode {
                underlying: format!("{:?}", e),
            },
        })
        .map(BagOfBytes::from)
        .map(CompiledNotarizedIntent::new)
}

impl HasSampleValues for CompiledNotarizedIntent {
    fn sample() -> Self {
        let bytes: BagOfBytes = "4d22030221022104210707010a872c0100000000000a912c01000000000009092f2400220101200720ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf01010800002022044103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c086c6f636b5f6665652101850000fda0c4277708000000000000000000000000000000004103800051c9a978fb5bfa066a3e5658251ee3304fb9bf58c35b61f8c10e0e7b91840c087769746864726177210280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a48000000000000000000000000000000000280005da66318c6318c61f5a61b4c6318c6318cf794aa8d295f14e6318c6318c6850000443945309a7a480000000000000000000000000000004103800051ac224ee242c339b5ea5f1ae567f0520a6ffa24b52a10b8e6cd96a8347f0c147472795f6465706f7369745f6f725f61626f72742102810000000022000020200022010121020c0a746578742f706c61696e2200010c0c48656c6c6f20526164697821202200220101210120074065938bf04b155de7277d95582ef2f5d36f7200765ee730cf3658da1861ad6e5008df90ac53d2835a48a5c0cb58891297761bda9533411e7eeddb1557d5dbe30a".parse().unwrap();

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

use sbor::ValueKind as ScryptoValueKind;
#[cfg(test)]
pub(crate) fn invalid_signed_intent() -> ScryptoSignedIntent {
    let invalid_value = ScryptoManifestValue::Tuple {
        fields: vec![ScryptoManifestValue::Array {
            element_value_kind: ScryptoValueKind::U8,
            elements: vec![
                ScryptoManifestValue::U8 { value: 1 },
                ScryptoManifestValue::U16 { value: 2 },
            ],
        }],
    };
    let dummy_address = ComponentAddress::with_node_id_bytes(
        &[0xffu8; 29],
        NetworkID::Stokenet,
    );
    let invalid_instruction = ScryptoInstruction::CallMethod {
        address: TryInto::<ScryptoDynamicComponentAddress>::try_into(
            &dummy_address,
        )
        .unwrap()
        .into(),
        method_name: "dummy".to_owned(),
        args: invalid_value,
    };
    ScryptoSignedIntent {
        intent: ScryptoIntent {
            header: TransactionHeader::sample().into(),
            instructions: ScryptoInstructions(vec![invalid_instruction]),
            blobs: ScryptoBlobs { blobs: Vec::new() },
            message: ScryptoMessage::None,
        },
        intent_signatures: ScryptoIntentSignatures {
            signatures: Vec::new(),
        },
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
        assert_eq!(SUT::sample().decompile(), NotarizedTransaction::sample());
        assert_eq!(
            SUT::sample_other().decompile(),
            NotarizedTransaction::sample_other()
        );
    }

    #[test]
    #[should_panic(
        expected = "Should never fail to decompile a 'CompiledNotarizedIntent' since we should not have been able to construct an invalid 'CompiledNotarizedIntent."
    )]
    fn decompile_fail() {
        _ = SUT {
            secret_magic: BagOfBytes::sample_aced(),
        }
        .decompile();
    }

    #[test]
    fn other_reasons_for_invalid() {
        let res = compile_notarized_intent(ScryptoNotarizedTransaction {
            signed_intent: invalid_signed_intent(),
            notary_signature: NotarySignature::sample().into(),
        });
        assert_eq!(
            res,
            Err(CommonError::InvalidNotarizedIntentFailedToEncode { underlying: "MismatchingArrayElementValueKind { element_value_kind: 7, actual_value_kind: 8 }".to_owned() }) 
        );
    }
}
