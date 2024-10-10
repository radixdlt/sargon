use crate::prelude::*;

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug("HDSignature {{ input: {:#?} }}", input)]
pub struct HDSignature {
    /// The input used to produce this `HDSignature`
    pub input: HDSignatureInput,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the HDFactorSource identified by
    /// `owned_hd_factor_
    /// instance.factor_s
    /// ource_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: Signature,
}

impl HDSignature {
    /// Constructs a HDSignature from an already produced `Signature`.
    pub(crate) fn with_details(
        input: HDSignatureInput,
        signature: Signature,
    ) -> Self {
        Self { input, signature }
    }

    pub fn intent_hash(&self) -> &IntentHash {
        &self.input.intent_hash
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

impl HasSampleValues for HDSignature {
    fn sample() -> Self {
        Self::sample_from_input(HDSignatureInput::sample())
    }

    fn sample_other() -> Self {
        Self::sample_from_input(HDSignatureInput::sample_other())
    }
}

impl HDSignature {
    // Signs with predefined mnemonics associated to the input's factor source id
    pub(crate) fn sample_from_input(input: HDSignatureInput) -> Self {
        let id = input.owned_factor_instance.factor_source_id();

        let mnemonic_with_passphrase = id.sample_associated_mnemonic();

        let signature = mnemonic_with_passphrase.sign(
            &input.intent_hash.hash,
            &input.owned_factor_instance.value.public_key.derivation_path,
        );

        HDSignature::with_details(input, signature.signature())
    }
}

#[cfg(test)]
impl HDSignature {
    pub fn produced_signing_with_input(input: HDSignatureInput) -> Self {
        Self::sample_from_input(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = HDSignature;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            IndexSet::<Sut>::from_iter([
                Sut::sample(),
                Sut::sample_other(),
                Sut::sample(),
                Sut::sample_other()
            ])
            .len(),
            2
        );
    }
}
