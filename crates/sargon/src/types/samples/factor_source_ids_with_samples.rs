use crate::prelude::*;

pub static ALL_FACTOR_SOURCE_ID_SAMPLES: Lazy<[FactorSourceIDFromHash; 11]> =
    Lazy::new(|| {
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

pub static MNEMONIC_BY_ID_MAP: Lazy<
    IndexMap<FactorSourceIDFromHash, MnemonicWithPassphrase>,
> = Lazy::new(|| {
    IndexMap::from_iter([
        (
            FactorSourceIDFromHash::sample_device(),
            MnemonicWithPassphrase::sample_device(),
        ),
        (
            FactorSourceIDFromHash::sample_ledger(),
            MnemonicWithPassphrase::sample_ledger(),
        ),
        (
            FactorSourceIDFromHash::sample_ledger_other(),
            MnemonicWithPassphrase::sample_ledger_other(),
        ),
        (
            FactorSourceIDFromHash::sample_arculus(),
            MnemonicWithPassphrase::sample_arculus(),
        ),
        (
            FactorSourceIDFromHash::sample_arculus_other(),
            MnemonicWithPassphrase::sample_arculus_other(),
        ),
        (
            FactorSourceIDFromHash::sample_passphrase(),
            MnemonicWithPassphrase::sample_passphrase(),
        ),
        (
            FactorSourceIDFromHash::sample_passphrase_other(),
            MnemonicWithPassphrase::sample_passphrase_other(),
        ),
        (
            FactorSourceIDFromHash::sample_off_device(),
            MnemonicWithPassphrase::sample_off_device(),
        ),
        (
            FactorSourceIDFromHash::sample_off_device_other(),
            MnemonicWithPassphrase::sample_off_device_other(),
        ),
        (
            FactorSourceIDFromHash::sample_security_questions(),
            MnemonicWithPassphrase::sample_security_questions(),
        ),
        (
            FactorSourceIDFromHash::sample_device_other(),
            MnemonicWithPassphrase::sample_device_other(),
        ),
    ])
});

impl FactorSourceIDFromHash {
    pub fn sample_at(index: usize) -> Self {
        ALL_FACTOR_SOURCE_ID_SAMPLES[index]
    }

    pub fn maybe_sample_associated_mnemonic(
        &self,
    ) -> Option<MnemonicWithPassphrase> {
        MNEMONIC_BY_ID_MAP.get(self).cloned()
    }

    pub fn sample_associated_mnemonic(&self) -> MnemonicWithPassphrase {
        self.maybe_sample_associated_mnemonic()
            .expect("Sample mnemonic with passphrase for id {} not found")
    }
}
