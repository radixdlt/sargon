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

    pub fn summary(&self, network_id: NetworkID) -> ManifestSummary {
        let ret_summary = RET_summary(&self.scrypto_manifest());
        ManifestSummary::from_ret(ret_summary, network_id)
    }

    pub fn execution_summary(
        &self,
        network_id: NetworkID,
        encoded_receipt: BagOfBytes, // TODO: Replace with TYPE - read from GW.
    ) -> Result<ExecutionSummary> {
        let receipt: TransactionReceipt = encoded_receipt.try_into()?;
        let ret_execution_summary =
            RET_execution_summary(&self.scrypto_manifest(), &receipt.0)
                .map_err(|e| {
                    error!(
                        "Failed to get execution summary from RET, error: {:?}",
                        e
                    );
                    CommonError::FailedToGetRetExecutionSummaryFromManifest
                })?;

        ExecutionSummary::from_ret(ret_execution_summary, network_id)
    }

    pub fn network_id(&self) -> NetworkID {
        self.secret_magic.instructions.network_id
    }

    pub fn resource_addresses_to_refresh(
        &self,
    ) -> Option<Vec<ResourceAddress>> {
        let (addresses, _) = RET_ins_extract_addresses(self.instructions());
        let resource_addresses: Vec<ResourceAddress> = addresses
            .into_iter()
            .filter_map(|a| {
                ResourceAddress::new(*a.as_node_id(), self.network_id()).ok()
            })
            .collect_vec();
        if resource_addresses.is_empty() {
            None
        } else {
            Some(resource_addresses)
        }
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
pub fn new_transaction_manifest_placeholder() -> TransactionManifest {
    TransactionManifest::placeholder_simulator()
}

#[uniffi::export]
pub fn new_transaction_manifest_placeholder_other() -> TransactionManifest {
    TransactionManifest::placeholder_simulator_other()
}

#[uniffi::export]
pub fn transaction_manifest_to_string(
    manifest: &TransactionManifest,
) -> String {
    // FIXME add blobs
    manifest.instructions_string()
}

impl HasPlaceholder for TransactionManifest {
    fn placeholder() -> Self {
        Self::placeholder_simulator()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_simulator_other()
    }
}

impl TransactionManifest {
    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/resource_transfer.rtm
    // but modified, changed `None` -> `Enum<0u8>()`
    fn placeholder_simulator_instructions_string() -> String {
        r#"CALL_METHOD
    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
    "lock_fee"
    Decimal("500")
;
CALL_METHOD
    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
    "withdraw"
    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
    Decimal("100")
;
CALL_METHOD
    Address("account_sim1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5syrgz9")
    "try_deposit_batch_or_abort"
    Expression("ENTIRE_WORKTOP")
    Enum<0u8>()
;
"#
.to_owned()
    }

    pub fn placeholder_simulator() -> Self {
        Self::new(
            Self::placeholder_simulator_instructions_string(),
            NetworkID::Simulator,
            Vec::new(),
        )
        .expect("Valid placeholder value")
    }

    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/multi_account_resource_transfer.rtm
    // but modified, changed `None` -> `Enum<0u8>()`, also changed `"account_a_bucket"` -> `"bucket1"`, `"account_b_bucket"` -> `"bucket2"`, etc.
    fn placeholder_other_simulator_instructions_string() -> String {
        r#"CALL_METHOD
    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
    "lock_fee"
    Decimal("500")
;
CALL_METHOD
    Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q")
    "withdraw"
    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
    Decimal("330")
;
TAKE_FROM_WORKTOP
    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
    Decimal("150")
    Bucket("bucket1")
;
CALL_METHOD
    Address("account_sim1c8mulhl5yrk6hh4jsyldps5sdrp08r5v9wusupvzxgqvhlp4c4nwjz")
    "try_deposit_or_abort"
    Bucket("bucket1")
    Enum<0u8>()
;
TAKE_FROM_WORKTOP
    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
    Decimal("130")
    Bucket("bucket2")
;
CALL_METHOD
    Address("account_sim1c8s2hass5g62ckwpv78y8ykdqljtetv4ve6etcz64gveykxznj36tr")
    "try_deposit_or_abort"
    Bucket("bucket2")
    Enum<0u8>()
;
TAKE_FROM_WORKTOP
    Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
    Decimal("50")
    Bucket("bucket3")
;
CALL_METHOD
    Address("account_sim1c8ct6jdcwqrg3gzskyxuy0z933fe55fyjz6p56730r95ulzwl3ppva")
    "try_deposit_or_abort"
    Bucket("bucket3")
    Enum<0u8>()
;
"#
.to_owned()
    }

    pub fn placeholder_simulator_other() -> Self {
        Self::new(
            Self::placeholder_other_simulator_instructions_string(),
            NetworkID::Simulator,
            Vec::new(),
        )
        .expect("Valid placeholder value")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    impl FromStr for TransactionManifest {
        type Err = crate::CommonError;

        fn from_str(s: &str) -> Result<Self> {
            Self::new(s, NetworkID::Simulator, Vec::new())
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TransactionManifest;

    #[test]
    fn placeholder_string_roundtrip() {
        let sut = SUT::placeholder();
        assert_eq!(sut.clone(), sut.clone());
        assert_eq!(
            SUT::placeholder_simulator_instructions_string(),
            sut.clone().instructions_string()
        );
        assert_eq!(sut.instructions().len(), 3);
    }

    #[test]
    fn placeholder_other_string_roundtrip() {
        let sut = SUT::placeholder_other();
        assert_eq!(sut.clone(), sut.clone());
        assert_eq!(
            SUT::placeholder_other_simulator_instructions_string(),
            sut.clone().instructions_string()
        );
        assert_eq!(sut.instructions().len(), 8);
    }
}
