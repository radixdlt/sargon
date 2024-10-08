use aes_gcm::aead::Payload;
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
    pub(crate) fn with_details(input: HDSignatureInput, signature: Signature) -> Self {
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

fn sample(
    payload: IntentHash,
    kind: FactorSourceKind,
    mnemonic_with_passphrase: MnemonicWithPassphrase,
    index: HDPathComponent
) -> HDSignature {
    let account_path = AccountPath::new(
        NetworkID::Mainnet,
        CAP26KeyKind::TransactionSigning,
        index.index(),
    );

    let factor_source_id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
        kind,
        &mnemonic_with_passphrase
    );

    let seed = mnemonic_with_passphrase.to_seed();
    let hd_private_key = seed.derive_private_key(&account_path);

    let hd_factor_instance = HierarchicalDeterministicFactorInstance::new(
        factor_source_id,
        hd_private_key.public_key(),
    );

    let factor_instance: HDFactorInstanceTransactionSigning<AccountPath> =
        HDFactorInstanceTransactionSigning::new(hd_factor_instance.clone()).unwrap();

    let account_address = AccountAddress::from_hd_factor_instance_virtual_entity_creation(
        factor_instance.clone(),
    );

    let signature_with_pub_key = mnemonic_with_passphrase.sign(
        &payload.hash,
        &hd_factor_instance.public_key.derivation_path
    );

    let hd_input = HDSignatureInput::new(
        payload,
        OwnedFactorInstance::new(
            AddressOfAccountOrPersona::Account(account_address),
            hd_factor_instance
        )
    );


    HDSignature::with_details(hd_input, signature_with_pub_key.signature())
}

impl HasSampleValues for HDSignature {
    fn sample() -> Self {
        sample(
            IntentHash::sample(),
            FactorSourceKind::Device,
            MnemonicWithPassphrase::sample_device(),
            HDPathComponent::from(0)
        )
    }

    fn sample_other() -> Self {
        sample(
            IntentHash::sample_other(),
            FactorSourceKind::Device,
            MnemonicWithPassphrase::sample_device(),
            HDPathComponent::from(0)
        )
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
