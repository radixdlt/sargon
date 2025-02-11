use crate::prelude::*;

/// An enum with the input to perform a spot check for a given `FactorSourceID`.
/// This is, to validate that the `FactorSourceID` was created with the same input that has been provided.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum SpotCheckInput {
    /// The user retrieved the id of a Ledger device.
    /// Used for the identification of `LedgerHardwareWalletFactorSource`.
    Ledger { id: Exactly32Bytes },

    /// The user retrieved the `FactorSourceIdFromHash` that identified an Arculus card.
    /// Used for the identification of `ArculusCardFactorSource`.
    ArculusCard { id: FactorSourceIDFromHash },

    /// The user retrieved a `MnemonicWithPassphrase`.
    /// Used for the identification of any software `FactorSource`.
    Software {
        mnemonic_with_passphrase: MnemonicWithPassphrase,
    },
}

pub trait FactorSourceIDSpotCheck {
    /// Performs a spot check and returns whether the `FactorSourceID` was created with the same input that has been provided.
    /// Returns `Err` when the input is not valid for the `FactorSourceKind`.
    fn perform_spot_check(&self, input: SpotCheckInput) -> bool;
}

impl FactorSourceIDSpotCheck for FactorSourceIDFromHash {
    fn perform_spot_check(&self, input: SpotCheckInput) -> bool {
        let id_from_hash = *self;
        let kind = self.kind;
        match input.clone() {
            SpotCheckInput::Ledger { id } => {
                assert_eq!(
                    kind,
                    FactorSourceKind::LedgerHQHardwareWallet,
                    "Unexpected Ledger input for kind: {:?}",
                    kind
                );
                let built_id = FactorSourceIDFromHash::new(kind, id);
                built_id == id_from_hash
            }
            SpotCheckInput::ArculusCard { id } => {
                assert_eq!(
                    kind,
                    FactorSourceKind::ArculusCard,
                    "Unexpected ArculusCard input for kind: {:?}",
                    kind
                );
                id == id_from_hash
            }
            SpotCheckInput::Software {
                mnemonic_with_passphrase,
            } => {
                assert!(
                    kind.expects_software_spot_check_input(),
                    "Unexpected Software input for kind: {:?}",
                    kind
                );
                let built_id =
                    FactorSourceIDFromHash::from_mnemonic_with_passphrase(
                        kind,
                        &mnemonic_with_passphrase,
                    );
                built_id == id_from_hash
            }
        }
    }
}

impl FactorSourceIDSpotCheck for FactorSourceID {
    fn perform_spot_check(&self, input: SpotCheckInput) -> bool {
        match self {
            FactorSourceID::Hash { value } => value.perform_spot_check(input),
            FactorSourceID::Address { .. } => {
                panic!("Address FactorSourceID does not support spot check")
            }
        }
    }
}

impl FactorSourceIDSpotCheck for FactorSource {
    fn perform_spot_check(&self, input: SpotCheckInput) -> bool {
        self.id().perform_spot_check(input)
    }
}

impl FactorSourceKind {
    /// Returns whether the kind expects a `Software` input for spot check.
    fn expects_software_spot_check_input(&self) -> bool {
        match self {
            FactorSourceKind::Device
            | FactorSourceKind::OffDeviceMnemonic
            | FactorSourceKind::Password
            | FactorSourceKind::SecurityQuestions => true,
            FactorSourceKind::LedgerHQHardwareWallet
            | FactorSourceKind::ArculusCard
            | FactorSourceKind::TrustedContact => false,
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = FactorSourceID;

    #[test]
    fn spot_check__device__mvp_matches() {
        let sut = SUT::sample_device();
        let input = SpotCheckInput::Software {
            mnemonic_with_passphrase: MnemonicWithPassphrase::sample_device(), // same mvp
        };
        let result = sut.perform_spot_check(input);
        assert!(result);
    }

    #[test]
    fn spot_check__device__mvp_does_not_match() {
        let sut = SUT::sample_device();
        let input = SpotCheckInput::Software {
            mnemonic_with_passphrase:
                MnemonicWithPassphrase::sample_device_other(), // different mvp
        };
        let result = sut.perform_spot_check(input);
        assert!(!result);
    }

    #[test]
    #[should_panic(expected = "Unexpected Ledger input for kind: Device")]
    fn spot_check__device__wrong_input() {
        let sut = SUT::sample_device();
        let input = SpotCheckInput::Ledger {
            id: Exactly32Bytes::sample(),
        };
        let _ = sut.perform_spot_check(input);
    }

    #[test]
    fn spot_check__ledger__id_matches() {
        let bytes = Exactly32Bytes::sample();
        let id = FactorSourceIDFromHash::new(
            FactorSourceKind::LedgerHQHardwareWallet,
            bytes,
        );
        let sut = SUT::from(id);
        let input = SpotCheckInput::Ledger { id: bytes };
        let result = sut.perform_spot_check(input);
        assert!(result);
    }

    #[test]
    fn spot_check__ledger__id_does_not_match() {
        let bytes = Exactly32Bytes::sample();
        let id = FactorSourceIDFromHash::new(
            FactorSourceKind::LedgerHQHardwareWallet,
            Exactly32Bytes::sample_other(),
        );
        let sut = SUT::from(id);
        let input = SpotCheckInput::Ledger { id: bytes };
        let result = sut.perform_spot_check(input);
        assert!(!result);
    }

    #[test]
    #[should_panic(
        expected = "Unexpected ArculusCard input for kind: LedgerHQHardwareWallet"
    )]
    fn spot_check__ledger__wrong_input() {
        let sut = SUT::sample_ledger();
        let input = SpotCheckInput::ArculusCard {
            id: FactorSourceIDFromHash::sample(),
        };
        let _ = sut.perform_spot_check(input);
    }

    #[test]
    fn spot_check__arculus__id_matches() {
        let id = FactorSourceIDFromHash::sample_arculus();
        let sut = SUT::from(id);
        let input = SpotCheckInput::ArculusCard { id };
        let result = sut.perform_spot_check(input);
        assert!(result);
    }

    #[test]
    fn spot_check__arculus__id_does_not_match() {
        let id = FactorSourceIDFromHash::sample_arculus();
        let sut = SUT::from(FactorSourceIDFromHash::sample_arculus_other());
        let input = SpotCheckInput::ArculusCard { id };
        let result = sut.perform_spot_check(input);
        assert!(!result);
    }

    #[test]
    #[should_panic(
        expected = "Unexpected Software input for kind: ArculusCard"
    )]
    fn spot_check__arculus__wrong_input() {
        let input = SpotCheckInput::Software {
            mnemonic_with_passphrase: MnemonicWithPassphrase::sample(),
        };
        let _ = SUT::sample_arculus().perform_spot_check(input.clone());
    }

    #[test]
    fn kind_expects_software_spot_check_input() {
        assert!(FactorSourceKind::Device.expects_software_spot_check_input());
        assert!(FactorSourceKind::OffDeviceMnemonic
            .expects_software_spot_check_input());
        assert!(FactorSourceKind::Password.expects_software_spot_check_input());
        assert!(FactorSourceKind::SecurityQuestions
            .expects_software_spot_check_input());
        assert!(!FactorSourceKind::LedgerHQHardwareWallet
            .expects_software_spot_check_input());
        assert!(
            !FactorSourceKind::ArculusCard.expects_software_spot_check_input()
        );
        assert!(!FactorSourceKind::TrustedContact
            .expects_software_spot_check_input());
    }

    #[test]
    fn spot_check_traits() {
        let factor_source = FactorSource::sample_device();
        let id = factor_source.id();
        let id_from_hash = id.as_hash().cloned().unwrap();
        let input = SpotCheckInput::Software {
            mnemonic_with_passphrase: MnemonicWithPassphrase::sample_device(),
        };
        assert_eq!(
            id.perform_spot_check(input.clone()),
            factor_source.perform_spot_check(input.clone())
        );
        assert_eq!(
            id_from_hash.perform_spot_check(input.clone()),
            factor_source.perform_spot_check(input.clone())
        );
        assert_eq!(
            id.perform_spot_check(input.clone()),
            id_from_hash.perform_spot_check(input.clone())
        );
    }
}
