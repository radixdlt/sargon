use crate::prelude::*;

pub(crate) static ALL_FACTOR_SOURCE_ID_SAMPLES: Lazy<
    [FactorSourceIDFromHash; 12],
> = Lazy::new(|| {
    [
        FactorSourceIDFromHash::sample_device(),
        FactorSourceIDFromHash::sample_ledger(),
        FactorSourceIDFromHash::sample_ledger_other(),
        FactorSourceIDFromHash::sample_arculus(),
        FactorSourceIDFromHash::sample_arculus_other(),
        FactorSourceIDFromHash::sample_password(),
        FactorSourceIDFromHash::sample_password_other(),
        FactorSourceIDFromHash::sample_off_device(),
        FactorSourceIDFromHash::sample_off_device_other(),
        FactorSourceIDFromHash::sample_security_questions(),
        FactorSourceIDFromHash::sample_device_other(),
        FactorSourceIDFromHash::sample_security_questions_other(),
    ]
});

pub(crate) static MNEMONIC_BY_ID_MAP: Lazy<
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
            FactorSourceIDFromHash::sample_password(),
            MnemonicWithPassphrase::sample_password(),
        ),
        (
            FactorSourceIDFromHash::sample_password_other(),
            MnemonicWithPassphrase::sample_password_other(),
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
            FactorSourceIDFromHash::sample_security_questions_other(),
            MnemonicWithPassphrase::sample_security_questions_other(),
        ),
        (
            FactorSourceIDFromHash::sample_device_other(),
            MnemonicWithPassphrase::sample_device_other(),
        ),
        (
            FactorSourceIDFromHash::sample_device_12_words(),
            MnemonicWithPassphrase::sample_device_12_words(),
        ),
        (
            FactorSourceIDFromHash::sample_device_12_words_other(),
            MnemonicWithPassphrase::sample_device_12_words_other(),
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
        self.maybe_sample_associated_mnemonic().unwrap_or_else(|| {
            panic!("Sample mnemonic with passphrase for id {} not found", self)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stable_id_of_security_questions() {
        assert_eq!(
            FactorSource::sample_security_questions_other().id_from_hash(),
            FactorSourceIDFromHash::sample_security_questions_other(),
        );
    }

    #[test]
    fn test_id_of_sample_factors_matches_keyed_values_id() {
        for (key, value) in MNEMONIC_BY_ID_MAP.iter() {
            let kind = key.kind;
            let id_of_keyed_value =
                FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                    kind, value,
                );
            assert_eq!(key, &id_of_keyed_value);
        }
    }
}
