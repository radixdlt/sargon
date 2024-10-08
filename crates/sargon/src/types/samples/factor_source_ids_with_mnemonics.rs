use crate::prelude::*;

pub(crate) static ALL_FACTOR_SOURCE_ID_SAMPLES: Lazy<[FactorSourceIDFromHash; 11]> = Lazy::new(|| {
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

fn mnemonic_from_fs_id(id: &FactorSourceIDFromHash) -> MnemonicWithPassphrase {
    let id = id.clone();
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
    } else if id ==  FactorSourceIDFromHash::sample_passphrase() {
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
        panic!("Sample mnemonic with passphrase for id {} not found", id.body.to_hex())
    }
}

pub fn fs_id_mnemonic(index: usize) -> (FactorSourceIDFromHash, MnemonicWithPassphrase) {
    let fs_id = ALL_FACTOR_SOURCE_ID_SAMPLES[index].clone();
    (fs_id, mnemonic_from_fs_id(&fs_id))
}

pub fn hd_factor_instance(
    id: &FactorSourceIDFromHash,
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    index: HDPathComponent
) -> HierarchicalDeterministicFactorInstance {
    let account_path = AccountPath::new(
        NetworkID::Mainnet,
        CAP26KeyKind::TransactionSigning,
        index.index(),
    );

    let seed = mnemonic_with_passphrase.to_seed();
    let hd_private_key = seed.derive_private_key(&account_path);

    HierarchicalDeterministicFactorInstance::new(
        id.clone(),
        hd_private_key.public_key(),
    )
}

pub fn hd_signature(
    id: &FactorSourceIDFromHash,
    mnemonic_with_passphrase: &MnemonicWithPassphrase,
    payload: IntentHash,
    index: HDPathComponent
) -> HDSignature {
    let hd_factor_instance = hd_factor_instance(
        id,
        mnemonic_with_passphrase,
        index
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


