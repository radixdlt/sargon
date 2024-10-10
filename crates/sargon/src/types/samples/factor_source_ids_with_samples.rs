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
}
