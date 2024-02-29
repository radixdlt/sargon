use std::ops::Deref;

use crate::prelude::*;

use radix_engine::transaction::TransactionReceipt as ScryptoTransactionReceipt;
use radix_engine_common::data::scrypto::scrypto_decode;
use radix_engine_common::network::NetworkDefinition as ScryptoNetworkDefinition;
use radix_engine_toolkit::functions::instructions::extract_addresses as RET_ins_extract_addresses;
use radix_engine_toolkit::functions::manifest::{
    execution_summary as RET_execution_summary, summary as RET_summary,
};
use transaction::{
    manifest::compile as scrypto_compile,
    manifest::decompile as scrypto_decompile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::{
        InstructionV1 as ScryptoInstruction,
        ManifestBuilder as ScryptoManifestBuilder,
        TransactionManifestV1 as ScryptoTransactionManifest,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record, derive_more::Display)]
#[display("{}", self.instructions_string())] // TODO add blobs
pub struct TransactionManifest {
    secret_magic: TransactionManifestSecretMagic,
}

impl From<TransactionManifestSecretMagic> for TransactionManifest {
    fn from(value: TransactionManifestSecretMagic) -> Self {
        Self {
            secret_magic: value,
        }
    }
}

impl TransactionManifest {
    fn scrypto_manifest(&self) -> ScryptoTransactionManifest {
        ScryptoTransactionManifest {
            instructions: self.instructions().clone(),
            blobs: self
                .secret_magic
                .blobs
                .clone()
                .into_iter()
                .map(|b| b.to_vec())
                .map(|blob| (hash_of(blob.clone()), blob))
                .collect(),
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
                blobs: scrypto_manifest
                    .blobs
                    .clone()
                    .values()
                    .map(|b| b.to_owned().into())
                    .collect_vec(),
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

    pub fn execution_summary(
        &self,
        encoded_receipt: BagOfBytes, // is: Vec<u8>
    ) -> Result<ExecutionSummary> {
        let receipt: TransactionReceipt = encoded_receipt.try_into()?;
        let ret_execution_summary =
            RET_execution_summary(&self.scrypto_manifest(), &receipt.decoded)
                .map_err(|e| {
                error!(
                    "Failed to get execution summary from RET, error: {:?}",
                    e
                );
                CommonError::FailedToGetRetExecutionSummaryFromManifest
            })?;
        ExecutionSummary::from_ret(ret_execution_summary, self.network_id())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use std::collections::BTreeMap;

    impl FromStr for TransactionManifest {
        type Err = crate::CommonError;

        fn from_str(s: &str) -> Result<Self> {
            Self::new(s, NetworkID::Simulator, Vec::new())
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
                Vec::new(),
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
            SUT::new(instructions_str, NetworkID::Simulator, Blobs::new())
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
            SUT::new(instructions_str, NetworkID::Mainnet, Blobs::new()),
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
            SUT::new(instructions_str, NetworkID::Stokenet, Blobs::new()),
            Err(CommonError::InvalidInstructionsWrongNetwork {
                found_in_instructions: NetworkID::Mainnet,
                specified_to_instructions_ctor: NetworkID::Stokenet
            })
        );
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn execution_summary() {
        let instructions_string = "CALL_METHOD Address(\"account_tdx_2_128h2zv5m4mnprnfjxn4nf96pglgx064mut8np26hp7w9mm064es2dn\") \"withdraw\" Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\") Decimal(\"123\"); TAKE_FROM_WORKTOP Address(\"resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc\") Decimal(\"123\") Bucket(\"bucket1\"); CALL_METHOD Address(\"account_tdx_2_128x8q5es2dstqtcc8wqm843xdtfs0lgetfcdn62a54wxspj6yhpxkf\") \"try_deposit_or_abort\" Bucket(\"bucket1\") Enum<0u8>();";
        let manifest =
            SUT::new(instructions_string, NetworkID::Stokenet, Blobs::new())
                .unwrap();

        let summary = manifest
            .execution_summary(TransactionReceipt::sample().encoded)
            .unwrap();

        assert_eq!(
            summary.detailed_manifest_class,
            vec![
                DetailedManifestClass::Transfer,
                DetailedManifestClass::General
            ]
        );
    }

    #[test]
    fn execution_summary_invalid_receipt() {
        assert_eq!(
            TransactionManifest::sample()
                .execution_summary(BagOfBytes::from_hex("dead").unwrap()),
            Err(CommonError::FailedToDecodeEncodedReceipt)
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
                Blobs::new()
            )
            .unwrap()
            .instructions_string(),
            s
        );
    }
}
