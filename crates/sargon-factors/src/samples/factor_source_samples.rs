use sargon_core_short_string::prelude::DisplayName;
use sargon_encryption::EncryptionScheme;

use crate::prelude::*;

#[allow(dead_code)]
pub(crate) static ALL_FACTOR_SOURCE_SAMPLES: Lazy<[FactorSource; 12]> =
    Lazy::new(|| {
        crate::samples::ALL_FACTOR_SOURCE_ID_SAMPLES
            .iter()
            .map(FactorSource::sample_from)
            .collect::<Vec<FactorSource>>()
            .try_into()
            .unwrap()
    });

impl FactorSource {
    pub fn sample_at(index: usize) -> FactorSource {
        ALL_FACTOR_SOURCE_SAMPLES[index].clone()
    }

    pub fn sample_all() -> IndexSet<FactorSource> {
        IndexSet::from_iter(ALL_FACTOR_SOURCE_SAMPLES.clone())
    }

    fn sample_from(id: &FactorSourceIDFromHash) -> Self {
        match id.kind {
            FactorSourceKind::LedgerHQHardwareWallet => {
                LedgerHardwareWalletFactorSource::new(
                    *id,
                    FactorSourceCommon::sample(),
                    LedgerHardwareWalletHint::new(
                        format!("Ledger @ {}", id.body.to_hex()).as_str(),
                        LedgerHardwareWalletModel::sample(),
                    ),
                )
                .into()
            }
            FactorSourceKind::ArculusCard => ArculusCardFactorSource::new(
                *id,
                ArculusCardHint::new(
                    format!("Arculus @ {}", id.body.to_hex()).as_str(),
                    ArculusCardModel::ArculusColdStorageWallet,
                ),
            )
            .into(),
            FactorSourceKind::Password => PasswordFactorSource::new(
                *id,
                PasswordFactorSourceHint::new(
                    format!("Password @ {}", id.body.to_hex()).as_str(),
                ),
            )
            .into(),
            FactorSourceKind::SecurityQuestions => {
                let sealed_mnemonic = SecurityQuestionsSealed_NOT_PRODUCTION_READY_Mnemonic::new_by_encrypting(
                    id.sample_associated_mnemonic().mnemonic,
                    Security_NOT_PRODUCTION_READY_QuestionsAndAnswers::sample(),
                    SecurityQuestions_NOT_PRODUCTION_READY_KDFScheme::default(),
                    EncryptionScheme::default(),
                ).unwrap();

                SecurityQuestions_NOT_PRODUCTION_READY_FactorSource::with_details(
                    *id,
                    FactorSourceCommon::sample(),
                    sealed_mnemonic
                ).into()
            }
            FactorSourceKind::OffDeviceMnemonic => {
                OffDeviceMnemonicFactorSource::new(
                    *id,
                    OffDeviceMnemonicHint::new(
                        DisplayName::new(format!(
                            "Off Device Mnemonic @ {}",
                            id.body.to_hex()
                        ))
                        .unwrap(),
                    ),
                )
                .into()
            }
            FactorSourceKind::Device => DeviceFactorSource::new(
                *id,
                FactorSourceCommon::sample(),
                DeviceFactorSourceHint::new(
                    format!("Device Label @ {}", id.body.to_hex()),
                    format!("Device Name @ {}", id.body.to_hex()),
                    format!("Device Model @ {}", id.body.to_hex()),
                    None,
                    None,
                    None,
                    id.sample_associated_mnemonic().mnemonic.word_count,
                ),
            )
            .into(),
            FactorSourceKind::TrustedContact => {
                panic!("Trusted contact is not supported in sample tests")
            }
        }
    }
}
