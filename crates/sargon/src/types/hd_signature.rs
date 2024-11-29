use crate::prelude::*;

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug("HDSignature {{ input: {:#?} }}", input)]
pub struct HDSignature<ID: SignableID> {
    /// The input used to produce this `HDSignature`
    pub input: HDSignatureInput<ID>,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the HDFactorSource identified by
    /// `owned_hd_factor_
    /// instance.factor_s
    /// ource_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: Signature,
}

impl<ID: SignableID> HDSignature<ID> {
    /// Constructs a HDSignature from an already produced `Signature`.
    pub fn new(
        input: HDSignatureInput<ID>,
        signature: Signature,
    ) -> Result<Self> {
        if !input.owned_factor_instance.value.public_key().is_valid_signature_for_hash(
            signature,
            &Into::<Hash>::into(input.payload_id.clone())
        ) {
            match input.owned_factor_instance.value.public_key.public_key {
                PublicKey::Ed25519(_) => {
                    Err(CommonError::SignaturePublicKeyCurveDiscrepancy {
                        expected_curve: SLIP10Curve::Curve25519.to_string()
                    })
                }
                PublicKey::Secp256k1(_) => {
                    Err(CommonError::SignaturePublicKeyCurveDiscrepancy {
                        expected_curve: SLIP10Curve::Secp256k1.to_string()
                    })
                }
            }
        } else {
            Ok(Self { input, signature })
        }
    }

    pub fn payload_id(&self) -> &ID {
        &self.input.payload_id
    }

    pub fn owned_factor_instance(&self) -> &OwnedFactorInstance {
        &self.input.owned_factor_instance
    }

    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.owned_factor_instance()
            .factor_instance()
            .factor_source_id
    }

    pub fn derivation_path(&self) -> DerivationPath {
        self.input
            .owned_factor_instance
            .factor_instance()
            .derivation_path()
    }
}

impl<ID: SignableID> From<&HDSignature<ID>> for SignatureWithPublicKey {
    fn from(value: &HDSignature<ID>) -> Self {
        let sig = value.signature;
        let pub_key = value
            .owned_factor_instance()
            .factor_instance()
            .public_key
            .clone();

        match sig {
            Signature::Secp256k1 { value } => {
                let pub_key = pub_key.public_key
                    .as_secp256k1()
                    .cloned()
                    .expect("Should never fail. The combination of signature and public key is validated at construction");
                SignatureWithPublicKey::from((pub_key, value))
            }
            Signature::Ed25519 { value } => {
                let pub_key = pub_key.public_key
                    .as_ed25519()
                    .cloned()
                    .expect("Should never fail. The combination of signature and public key is validated at construction");
                SignatureWithPublicKey::from((pub_key, value))
            }
        }
    }
}

impl<ID: SignableID> HasSampleValues for HDSignature<ID> {
    fn sample() -> Self {
        Self::fake_sign_by_looking_up_mnemonic_amongst_samples(
            HDSignatureInput::<ID>::sample(),
        )
    }

    fn sample_other() -> Self {
        Self::fake_sign_by_looking_up_mnemonic_amongst_samples(
            HDSignatureInput::<ID>::sample_other(),
        )
    }
}

impl<ID: SignableID> HDSignature<ID> {
    /// WARNING: Should only be used in samples and unit tests
    ///
    /// Signs with predefined mnemonics associated to the input's factor source id
    pub fn fake_sign_by_looking_up_mnemonic_amongst_samples(
        input: HDSignatureInput<ID>,
    ) -> Self {
        let id = input.owned_factor_instance.factor_source_id();

        let mnemonic_with_passphrase = id.sample_associated_mnemonic();

        let signature = mnemonic_with_passphrase.sign(
            &input.payload_id.clone().into(),
            &input.owned_factor_instance.value.public_key.derivation_path,
        );

        HDSignature::new(input, signature.signature())
            .expect("Sign method should produce signatures using the correct curve from public keys")
    }
}

#[cfg(test)]
impl<ID: SignableID> HDSignature<ID> {
    pub fn produced_signing_with_input(input: HDSignatureInput<ID>) -> Self {
        Self::fake_sign_by_looking_up_mnemonic_amongst_samples(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HDSignature<TransactionIntentHash>;

    #[test]
    fn equality_of_samples() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<SUT>::from_iter([
                SUT::sample(),
                SUT::sample_other(),
                SUT::sample(),
                SUT::sample_other()
            ])
            .len(),
            2
        );
    }

    #[test]
    fn new_fails_due_to_discrepancy_on_signature_curve() {
        let ed25519_based_input = HDSignatureInput::<TransactionIntentHash>::sample();
        let secp256k1_signature = Signature::from(Secp256k1Signature::sample());

        assert_eq!(
            HDSignature::new(ed25519_based_input, secp256k1_signature),
            Err(CommonError::SignaturePublicKeyCurveDiscrepancy {
                expected_curve: SLIP10Curve::Curve25519.to_string()
            })
        );
    }

    #[test]
    fn into_signature_with_public_key() {
        let sut = SUT::sample();

        let signature_with_public_key = SignatureWithPublicKey::from(&sut);

        assert_eq!(signature_with_public_key.signature(), sut.signature);
        assert_eq!(
            signature_with_public_key.public_key(),
            sut.input
                .owned_factor_instance
                .factor_instance()
                .public_key()
        );
    }
}
