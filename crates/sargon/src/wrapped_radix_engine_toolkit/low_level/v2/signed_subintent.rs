use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
pub struct SignedSubintent {
    pub subintent: Subintent,
    pub subintent_signatures: IntentSignatures,
}

impl SignedSubintent {
    pub fn new(
        subintent: Subintent,
        subintent_signatures: IntentSignatures,
    ) -> Result<Self> {
        if !subintent_signatures.validate(subintent.hash()) {
            return Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash);
        }

        // Verify that this SignedSubintent has acceptable depth and is compatible
        _ = compile_signed_subintent_with(&subintent, &subintent_signatures)?;

        Ok(Self {
            subintent,
            subintent_signatures,
        })
    }

    pub fn compiled(&self) -> Vec<u8> {
        compile_signed_subintent_with(
            &self.subintent,
            &self.subintent_signatures,
        )
        .expect("Compiling after intialization is always valid")
    }
}

fn into_scrypto(
    subintent: &Subintent,
    subintent_signatures: &IntentSignatures,
) -> ScryptoSignedPartialTransaction {
    ScryptoSignedPartialTransaction {
        partial_transaction: ScryptoPartialTransaction {
            root_subintent: ScryptoSubintent::from(subintent.clone()),
            non_root_subintents: ScryptoNonRootSubintents(vec![]),
        },
        root_subintent_signatures: ScryptoIntentSignaturesV2 {
            signatures: subintent_signatures
                .clone()
                .signatures
                .into_iter()
                .map(|s| s.into())
                .collect(),
        },
        non_root_subintent_signatures: ScryptoNonRootSubintentSignatures {
            by_subintent: vec![],
        },
    }
}

fn compile_signed_subintent_with(
    subintent: &Subintent,
    subintent_signatures: &IntentSignatures,
) -> Result<Vec<u8>> {
    compile_signed_subintent(into_scrypto(subintent, subintent_signatures))
}

fn compile_signed_subintent(
    signed_partial_transaciton: ScryptoSignedPartialTransaction,
) -> Result<Vec<u8>> {
    RET_compile_signed_partial_tx(&signed_partial_transaciton).map_err(|e| {
        match e {
            sbor::EncodeError::MaxDepthExceeded(max) => {
                CommonError::InvalidTransactionMaxSBORDepthExceeded {
                    max: max as u16,
                }
            }
            _ => CommonError::InvalidSignedIntentFailedToEncode {
                underlying: format!("{:?}", e),
            },
        }
    })
}

impl From<SignedSubintent> for ScryptoSignedPartialTransaction {
    fn from(val: SignedSubintent) -> Self {
        into_scrypto(&val.subintent, &val.subintent_signatures)
    }
}

impl HasSampleValues for SignedSubintent {
    fn sample() -> Self {
        let intent = Subintent::sample();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature =
                private_key.sign_subintent_hash(&intent.hash());
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);

        Self::new(intent, intent_signatures).unwrap()
    }

    fn sample_other() -> Self {
        Self::new(Subintent::sample_other(), IntentSignatures::default())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SignedSubintent;

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
    fn many_intent_signatures_all_valid() {
        let intent = Subintent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature =
                private_key.sign_subintent_hash(&intent.hash());
            signatures.push(intent_signature)
        }

        let intent_signatures = IntentSignatures::new(signatures);
        assert_eq!(
            intent_signatures.clone().signatures.into_iter().map(|s| s.signature().to_string()).collect_vec(),
            [
                "014f6e14706f07aaa632ed83a4df6ee76c3bfb0693af6bd0c010a49dd2071b1c910c26143e501f7cca5c7aba6e8db8b4221a0a9b3703d38a20257fdbb38eb2920e",
                "00c23163dd81b1648d101b69c571bc0cad9e9da3ac9309f1dc0f40b01c02fb17dc5b77a2bd569f98fea088c419cd9d3b1a27f094817da6b08f420d9d36777b0b84",
                "00d50da65a9423f14bfda39589c8ccd1cce7c191e064918a92b82891e8e7aaa4853ede0a8ce659ce63f5c0160f72b34b77ca52ba7b519db48da51f7bd8f470ae6e",
            ]
        );

        assert_eq!(
            SUT::new(intent, intent_signatures.clone())
                .unwrap()
                .subintent_signatures,
            intent_signatures
        );
    }

    #[test]
    fn many_intent_signatures_one_invalid() {
        let intent = Subintent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();

            let intent_signature =
                private_key.sign_subintent_hash(&intent.hash());
            signatures.push(intent_signature)
        }

        signatures.push(IntentSignature::sample());

        assert_eq!(
            SUT::new(intent, IntentSignatures::new(signatures)),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }

    #[test]
    fn many_intent_signatures_invalid_because_mismatching_intent() {
        let intent = Subintent::sample_other();

        let mut signatures = Vec::<IntentSignature>::new();
        for n in 1..4 {
            let private_key: Secp256k1PrivateKey =
                ScryptoSecp256k1PrivateKey::from_u64(n).unwrap().into();
            let hash = intent.hash();
            let intent_signature = private_key.sign_subintent_hash(&hash);
            signatures.push(intent_signature)
        }

        assert_eq!(
            SUT::new(
                Subintent::sample(), // <-- WRONG Intent, not was signed.
                IntentSignatures::new(signatures)
            ),
            Err(CommonError::InvalidSignaturesForIntentSomeDidNotValidateIntentHash)
        );
    }
}
