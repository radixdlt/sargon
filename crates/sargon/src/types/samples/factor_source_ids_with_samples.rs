use crate::prelude::*;

pub(crate) static ALL_FACTOR_SOURCE_ID_SAMPLES: Lazy<
    [FactorSourceIDFromHash; 11],
> = Lazy::new(|| {
    [
        FactorSourceIDFromHash::sample_device(),
        FactorSourceIDFromHash::sample_ledger(),
        FactorSourceIDFromHash::sample_ledger_other(),
        FactorSourceIDFromHash::sample_arculus(),
        FactorSourceIDFromHash::sample_arculus_other(),
        FactorSourceIDFromHash::sample_passphrase(),
        FactorSourceIDFromHash::sample_passphrase_other(),
        FactorSourceIDFromHash::sample_off_device(),
        FactorSourceIDFromHash::sample_off_device_other(),
        FactorSourceIDFromHash::sample_security_questions(),
        FactorSourceIDFromHash::sample_device_other(),
    ]
});

impl FactorSourceIDFromHash {
    pub fn sample_at(index: usize) -> Self {
        ALL_FACTOR_SOURCE_ID_SAMPLES[index].clone()
    }

    pub fn sample_associated_mnemonic(&self) -> MnemonicWithPassphrase {
        let id = *self;
        if id == FactorSourceIDFromHash::sample_device() {
            MnemonicWithPassphrase::sample_device()
        } else if id == FactorSourceIDFromHash::sample_ledger() {
            MnemonicWithPassphrase::sample_ledger()
        } else if id == FactorSourceIDFromHash::sample_ledger_other() {
            MnemonicWithPassphrase::sample_ledger_other()
        } else if id == FactorSourceIDFromHash::sample_arculus() {
            MnemonicWithPassphrase::sample_arculus()
        } else if id == FactorSourceIDFromHash::sample_arculus_other() {
            MnemonicWithPassphrase::sample_arculus_other()
        } else if id == FactorSourceIDFromHash::sample_passphrase() {
            MnemonicWithPassphrase::sample_passphrase()
        } else if id == FactorSourceIDFromHash::sample_passphrase_other() {
            MnemonicWithPassphrase::sample_passphrase_other()
        } else if id == FactorSourceIDFromHash::sample_off_device() {
            MnemonicWithPassphrase::sample_off_device()
        } else if id == FactorSourceIDFromHash::sample_off_device_other() {
            MnemonicWithPassphrase::sample_off_device_other()
        } else if id == FactorSourceIDFromHash::sample_security_questions() {
            MnemonicWithPassphrase::sample_security_questions()
        } else if id == FactorSourceIDFromHash::sample_device_other() {
            MnemonicWithPassphrase::sample_device_other()
        } else {
            panic!(
                "Sample mnemonic with passphrase for id {} not found",
                id.body.to_hex()
            )
        }
    }

    pub fn sample_tx_factor_instance(
        &self,
        index: HDPathComponent,
        kind: CAP26EntityKind,
    ) -> HierarchicalDeterministicFactorInstance {
        let derivation_path: DerivationPath = match kind {
            CAP26EntityKind::Account => AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
            CAP26EntityKind::Identity => IdentityPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
        };

        let seed = self.sample_associated_mnemonic().to_seed();
        let hd_private_key = seed.derive_private_key(&derivation_path);

        HierarchicalDeterministicFactorInstance::new(
            self.clone(),
            hd_private_key.public_key(),
        )
    }

    pub fn sample_tx_hd_signature(
        &self,
        payload: IntentHash,
        index: HDPathComponent,
    ) -> HDSignature {
        let hd_factor_instance =
            self.sample_tx_factor_instance(index, CAP26EntityKind::Account);

        let factor_instance: HDFactorInstanceTransactionSigning<AccountPath> =
            HDFactorInstanceTransactionSigning::new(hd_factor_instance.clone())
                .unwrap();

        let account_address =
            AccountAddress::from_hd_factor_instance_virtual_entity_creation(
                factor_instance.clone(),
            );

        let signature_with_pub_key = self.sample_associated_mnemonic().sign(
            &payload.hash,
            &hd_factor_instance.public_key.derivation_path,
        );

        let hd_input = HDSignatureInput::new(
            payload,
            OwnedFactorInstance::new(
                AddressOfAccountOrPersona::Account(account_address),
                hd_factor_instance,
            ),
        );

        HDSignature::with_details(hd_input, signature_with_pub_key.signature())
    }
}
