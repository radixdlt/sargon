use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.instructions_string())] // TODO add blobs to Display
pub struct TransactionManifest {
    pub(crate) secret_magic: TransactionManifestSecretMagic,
}

impl TransactionManifest {
    pub fn with_instructions_and_blobs(
        instructions: Instructions,
        blobs: Blobs,
    ) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagic::new(
                instructions,
                blobs,
            ),
        }
    }
}

impl TransactionManifest {
    pub(crate) fn empty(network_id: NetworkID) -> Self {
        Self {
            secret_magic: TransactionManifestSecretMagic {
                instructions: Instructions::empty(network_id),
                blobs: Blobs::default(),
            },
        }
    }
}

impl From<TransactionManifestSecretMagic> for TransactionManifest {
    fn from(value: TransactionManifestSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl TransactionManifest {
    pub(crate) fn scrypto_manifest(&self) -> ScryptoTransactionManifest {
        ScryptoTransactionManifest {
            instructions: self.instructions().clone(),
            blobs: self.secret_magic.blobs.clone().into(),
        }
    }
}

impl From<TransactionManifest> for ScryptoTransactionManifest {
    fn from(value: TransactionManifest) -> Self {
        value.scrypto_manifest()
    }
}

impl TransactionManifest {
    pub(crate) fn instructions(&self) -> &Vec<ScryptoInstruction> {
        &self.secret_magic.instructions.secret_magic.0
    }

    pub(crate) fn blobs(&self) -> &Blobs {
        &self.secret_magic.blobs
    }

    pub(crate) fn from_scrypto(
        scrypto_manifest: ScryptoTransactionManifest,
        network_id: NetworkID,
    ) -> Self {
        let value = Self {
            secret_magic: TransactionManifestSecretMagic {
                instructions: Instructions {
                    secret_magic: InstructionsSecretMagic(
                        scrypto_manifest.instructions.clone(),
                    ),
                    network_id,
                },
                blobs: scrypto_manifest.blobs.clone().into(),
            },
        };
        assert_eq!(value.scrypto_manifest(), scrypto_manifest);
        value
    }

    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Result<Self> {
        Instructions::new(instructions_string, network_id).map(|instructions| {
            Self {
                secret_magic: TransactionManifestSecretMagic {
                    instructions,
                    blobs,
                },
            }
        })
    }

    pub fn instructions_string(&self) -> String {
        self.secret_magic.instructions.instructions_string()
    }

    pub fn summary(&self) -> ManifestSummary {
        let ret_summary = RET_summary(&self.scrypto_manifest());
        ManifestSummary::from_ret(ret_summary, self.network_id())
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn resource_addresses_to_refresh(&self) -> Vec<ResourceAddress> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec()
    }
}

#[uniffi::export]
pub fn new_transaction_manifest_from_instructions_string_and_blobs(
    instructions_string: String,
    network_id: NetworkID,
    blobs: Blobs,
) -> Result<TransactionManifest> {
    TransactionManifest::new(instructions_string, network_id, blobs)
}

#[uniffi::export]
pub fn new_transaction_manifest_sample() -> TransactionManifest {
    TransactionManifest::sample()
}

#[uniffi::export]
pub fn new_transaction_manifest_sample_other() -> TransactionManifest {
    TransactionManifest::sample_other()
}

#[uniffi::export]
pub fn transaction_manifest_to_string(
    manifest: &TransactionManifest,
) -> String {
    // FIXME add blobs
    manifest.instructions_string()
}

impl HasSampleValues for TransactionManifest {
    fn sample() -> Self {
        TransactionManifestSecretMagic::sample().into()
    }

    fn sample_other() -> Self {
        TransactionManifestSecretMagic::sample_other().into()
    }
}

#[allow(unused)]
impl TransactionManifest {
    pub(crate) fn sample_mainnet_without_lock_fee() -> Self {
        TransactionManifestSecretMagic::sample_mainnet_without_lock_fee().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::collections::BTreeMap;

    impl FromStr for TransactionManifest {
        type Err = crate::CommonError;

        fn from_str(s: &str) -> Result<Self> {
            Self::new(s, NetworkID::Simulator, Blobs::default())
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn sample_string_roundtrip() {
        let sut = SUT::sample();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().secret_magic.instructions,
            Instructions::sample_mainnet_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 4);
    }

    #[test]
    fn sample_other_string_roundtrip() {
        let sut = SUT::sample_other();
        assert_eq!(sut.clone(), sut.clone());
        instructions_eq(
            sut.clone().secret_magic.instructions,
            Instructions::sample_other_simulator_instructions_string(),
        );
        assert_eq!(sut.instructions().len(), 8);
    }

    #[test]
    fn scrypto_roundtrip() {
        let ins = vec![
            ScryptoInstruction::DropAllProofs,
            ScryptoInstruction::DropAuthZoneProofs,
        ];
        let scrypto = ScryptoTransactionManifest {
            instructions: ins.clone(),
            blobs: BTreeMap::new(),
        };
        let sut = SUT {
            secret_magic: TransactionManifestSecretMagic::new(
                Instructions {
                    secret_magic: InstructionsSecretMagic(ins),
                    network_id: NetworkID::Mainnet,
                },
                Blobs::default(),
            ),
        };
        assert_eq!(scrypto.clone(), sut.clone().into());
        assert_eq!(sut.scrypto_manifest(), scrypto);
    }

    #[test]
    fn new_from_instructions_string() {
        let instructions_str = r#"CALL_METHOD
        Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Simulator, Blobs::default())
                .unwrap()
                .instructions()
                .len(),
            1
        );
    }

    #[test]
    fn new_from_instructions_string_wrong_network_id_sim_main() {
        let instructions_str = r#"CALL_METHOD
        Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Mainnet, Blobs::default()),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Simulator,
                specified_to_instructions_ctor: NetworkID::Mainnet
            })
        );
    }

    #[test]
    fn new_from_instructions_string_wrong_network_id_main_sim() {
        let instructions_str = r#"CALL_METHOD
        Address("account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease")
        "lock_fee"
        Decimal("500");
                "#;

        assert_eq!(
            SUT::new(instructions_str, NetworkID::Stokenet, Blobs::default()),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Stokenet
            })
        );
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample().network_id(), NetworkID::Mainnet);
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Simulator);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn manifest_summary() {
        let manifest = SUT::sample();
        let summary = manifest.summary();
        assert_eq!(summary.addresses_of_accounts_requiring_auth[0].address(), "account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease");
    }

    #[test]
    fn resource_addresses_to_refresh() {
        let manifest = SUT::sample();
        let resources = manifest.resource_addresses_to_refresh();
        assert_eq!(resources[0].address(), "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd");
    }
}

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn samples() {
        assert_eq!(new_transaction_manifest_sample(), SUT::sample());
        assert_eq!(
            new_transaction_manifest_sample_other(),
            SUT::sample_other()
        );
    }

    #[test]
    fn to_string() {
        assert_eq!(
            transaction_manifest_to_string(&SUT::sample()),
            SUT::sample().to_string()
        );
    }

    #[test]
    fn test_new_transaction_manifest_from_instructions_string_and_blobs() {
        let s = new_transaction_manifest_sample().instructions_string();

        assert_eq!(
            new_transaction_manifest_from_instructions_string_and_blobs(
                s.clone(),
                NetworkID::Mainnet,
                Blobs::default()
            )
            .unwrap()
            .instructions_string(),
            s
        );
    }
}
