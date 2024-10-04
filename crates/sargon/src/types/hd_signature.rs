use crate::prelude::*;
use radix_engine_toolkit::models::canonical_address_types::NetworkId;

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug("HDSignature {{ input: {:#?} }}", input)]
pub struct HDSignature {
    /// The input used to produce this `HDSignature`
    pub input: HDSignatureInput,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the HDFactorSource identified by
    /// `owned_hd_factor_instance.factor_source_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: Signature,
}

impl HDSignature {
    /// Constructs a HDSignature from an already produced `Signature`.
    fn with_details(input: HDSignatureInput, signature: Signature) -> Self {
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
        let mnemonic = MnemonicWithPassphrase::sample_device();
        let device_factor_source =
            DeviceFactorSource::babylon(true, &mnemonic, &HostInfo::sample());

        let private_hd_fs = PrivateHierarchicalDeterministicFactorSource::new(
            mnemonic,
            device_factor_source,
        );

        let hd_factor_instance = private_hd_fs
            .derive_entity_creation_factor_instance::<AccountPath>(
                NetworkID::Mainnet,
                HDPathValue::from(0u32),
            );

        let address = AccountAddress::from_public_key(
            hd_factor_instance.public_key,
            NetworkID::Mainnet,
        );

        let factor_instance = HierarchicalDeterministicFactorInstance::new(
            hd_factor_instance.factor_source_id,
            HierarchicalDeterministicPublicKey::new(
                hd_factor_instance.public_key,
                hd_factor_instance.path.derivation_path(),
            ),
        );

        let owned_fs = OwnedFactorInstance::owned_factor_instance(
            AddressOfAccountOrPersona::Account(address),
            factor_instance,
        );

        let input = HDSignatureInput::new(IntentHash::sample(), owned_fs);

        let signature_with_public_key =
            private_hd_fs.mnemonic_with_passphrase.sign(
                &input.intent_hash.hash,
                &hd_factor_instance.path.derivation_path(),
            );

        HDSignature::with_details(input, signature_with_public_key.signature())
    }

    fn sample_other() -> Self {
        let mnemonic = MnemonicWithPassphrase::sample_device();
        let device_factor_source =
            DeviceFactorSource::babylon(true, &mnemonic, &HostInfo::sample());

        let private_hd_fs = PrivateHierarchicalDeterministicFactorSource::new(
            mnemonic,
            device_factor_source,
        );

        let hd_factor_instance = private_hd_fs
            .derive_entity_creation_factor_instance::<AccountPath>(
                NetworkID::Mainnet,
                HDPathValue::from(0u32),
            );

        let address = AccountAddress::from_public_key(
            hd_factor_instance.public_key,
            NetworkID::Mainnet,
        );

        let factor_instance = HierarchicalDeterministicFactorInstance::new(
            hd_factor_instance.factor_source_id,
            HierarchicalDeterministicPublicKey::new(
                hd_factor_instance.public_key,
                hd_factor_instance.path.derivation_path(),
            ),
        );

        let owned_fs = OwnedFactorInstance::owned_factor_instance(
            AddressOfAccountOrPersona::Account(address),
            factor_instance,
        );

        let input = HDSignatureInput::new(IntentHash::sample_other(), owned_fs);

        let signature_with_public_key =
            private_hd_fs.mnemonic_with_passphrase.sign(
                &input.intent_hash.hash,
                &hd_factor_instance.path.derivation_path(),
            );

        HDSignature::with_details(input, signature_with_public_key.signature())
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
