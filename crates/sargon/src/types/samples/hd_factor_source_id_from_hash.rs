use crate::prelude::*;

/// A type used in tests to provide both the factor source id and the mnemonic with passphrase
/// it originated from
#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:#?} - {:?}", id, mnemonic_with_passphrase)]
pub struct HDFactorSourceIdFromHash {
    pub id: FactorSourceIDFromHash,
    pub mnemonic_with_passphrase: MnemonicWithPassphrase
}

impl HDFactorSourceIdFromHash {
    pub fn new(
        kind: FactorSourceKind,
        mnemonic_with_passphrase: MnemonicWithPassphrase
    ) -> Self {
        let id = FactorSourceIDFromHash::from_mnemonic_with_passphrase(
            kind,
            &mnemonic_with_passphrase
        );

        Self {
            id,
            mnemonic_with_passphrase,
        }
    }

    pub fn hd_factor_instance(
        &self,
        index: HDPathValue
    ) -> HierarchicalDeterministicFactorInstance {
        let account_path = AccountPath::new(
            NetworkID::Mainnet,
            CAP26KeyKind::TransactionSigning,
            index,
        );

        let seed = self.mnemonic_with_passphrase.to_seed();
        let hd_private_key = seed.derive_private_key(&account_path);

        HierarchicalDeterministicFactorInstance::new(
            self.id,
            hd_private_key.public_key(),
        )
    }

    pub fn hd_signature(
        &self,
        payload: IntentHash,
        index: HDPathComponent
    ) -> HDSignature {
        let hd_factor_instance = self.hd_factor_instance(index.index());

        let factor_instance: HDFactorInstanceTransactionSigning<AccountPath> =
            HDFactorInstanceTransactionSigning::new(hd_factor_instance.clone()).unwrap();

        let account_address = AccountAddress::from_hd_factor_instance_virtual_entity_creation(
            factor_instance.clone(),
        );

        let signature_with_pub_key = self.mnemonic_with_passphrase.sign(
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

    pub fn sample_device() -> Self {
        Self::new(
            FactorSourceKind::Device,
            MnemonicWithPassphrase::sample_device()
        )
    }

    pub fn sample_device_other() -> Self {
        Self::new(
            FactorSourceKind::Device,
            MnemonicWithPassphrase::sample_device_other()
        )
    }

    pub fn sample_ledger() -> Self {
        Self::new(
            FactorSourceKind::LedgerHQHardwareWallet,
            MnemonicWithPassphrase::sample_ledger()
        )
    }

    pub fn sample_ledger_other() -> Self {
        Self::new(
            FactorSourceKind::LedgerHQHardwareWallet,
            MnemonicWithPassphrase::sample_ledger_other()
        )
    }

    pub fn sample_arculus() -> Self {
        Self::new(
            FactorSourceKind::ArculusCard,
            MnemonicWithPassphrase::sample_arculus()
        )
    }

    pub fn sample_arculus_other() -> Self {
        Self::new(
            FactorSourceKind::ArculusCard,
            MnemonicWithPassphrase::sample_arculus_other()
        )
    }

    pub fn sample_off_device() -> Self {
        Self::new(
            FactorSourceKind::OffDeviceMnemonic,
            MnemonicWithPassphrase::sample_off_device()
        )
    }

    pub fn sample_off_device_other() -> Self {
        Self::new(
            FactorSourceKind::OffDeviceMnemonic,
            MnemonicWithPassphrase::sample_off_device_other()
        )
    }

    pub fn sample_security_questions() -> Self {
        Self::new(
            FactorSourceKind::SecurityQuestions,
            MnemonicWithPassphrase::sample_security_questions()
        )
    }

    pub fn sample_security_questions_other() -> Self {
        Self::new(
            FactorSourceKind::SecurityQuestions,
            MnemonicWithPassphrase::sample_security_questions_other()
        )
    }

    pub fn sample_passphrase() -> Self {
        Self::new(
            FactorSourceKind::Passphrase,
            MnemonicWithPassphrase::sample_passphrase()
        )
    }

    pub fn sample_passphrase_other() -> Self {
        Self::new(
            FactorSourceKind::Passphrase,
            MnemonicWithPassphrase::sample_passphrase_other()
        )
    }

    pub fn sample_at(index: usize) -> Self {
        ALL_HD_FACTOR_SOURCE_IDS[index].clone()
    }
}

pub(crate) static ALL_HD_FACTOR_SOURCE_IDS: Lazy<[HDFactorSourceIdFromHash; 11]> = Lazy::new(|| {
    [
        HDFactorSourceIdFromHash::sample_device(),
        HDFactorSourceIdFromHash::sample_ledger(),
        HDFactorSourceIdFromHash::sample_ledger_other(),
        HDFactorSourceIdFromHash::sample_arculus(),
        HDFactorSourceIdFromHash::sample_arculus_other(),
        HDFactorSourceIdFromHash::sample_passphrase(),
        HDFactorSourceIdFromHash::sample_passphrase_other(),
        HDFactorSourceIdFromHash::sample_off_device(),
        HDFactorSourceIdFromHash::sample_off_device_other(),
        HDFactorSourceIdFromHash::sample_security_questions(),
        HDFactorSourceIdFromHash::sample_device_other(),
    ]
});