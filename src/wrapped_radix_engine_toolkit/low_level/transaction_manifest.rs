use std::ops::Deref;

use crate::prelude::*;

use radix_engine::transaction::TransactionReceipt as ScryptoTransactionReceipt;
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

use radix_engine_common::data::scrypto::scrypto_decode;

pub type Blob = BagOfBytes;
pub type Blobs = Vec<Blob>;
pub type ScryptoInstructions = Vec<ScryptoInstruction>;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Object)]
pub struct ManifestInner {
    pub network_id: NetworkID,
    pub scrypto_manifest: ScryptoTransactionManifest,
}
impl Deref for ManifestInner {
    type Target = ScryptoTransactionManifest;

    fn deref(&self) -> &Self::Target {
        &self.scrypto_manifest
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Manifest {
    secret_magic: Arc<ManifestInner>,
}

impl From<ManifestInner> for Manifest {
    fn from(value: ManifestInner) -> Self {
        Self {
            secret_magic: Arc::new(value),
        }
    }
}

impl From<Manifest> for ScryptoTransactionManifest {
    fn from(value: Manifest) -> Self {
        value.secret_magic.scrypto_manifest.clone()
    }
}

impl Deref for Manifest {
    type Target = ScryptoTransactionManifest;

    fn deref(&self) -> &Self::Target {
        &self.secret_magic
    }
}

impl Manifest {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Result<Self> {
        Instructions::new(instructions_string, network_id)
            .map(|i| ScryptoTransactionManifest {
                instructions: i.0,
                blobs: blobs
                    .into_iter()
                    .map(|b| b.to_vec())
                    .collect_vec()
                    .iter()
                    .map(|blob| (hash_of(blob), blob.clone()))
                    .collect(),
            })
            .map(|scrypto_manifest| Self {
                secret_magic: ManifestInner {
                    network_id,
                    scrypto_manifest,
                }
                .into(),
            })
    }

    pub fn instructions_string(&self) -> String {
        let network_definition =
            self.secret_magic.network_id.network_definition();
        scrypto_decompile(&self.secret_magic.scrypto_manifest.instructions, &network_definition).expect("Should never fail, because should never have allowed invalid instructions")
    }

    /// This clones the blobs which might be expensive resource wise.
    pub fn blobs(&self) -> Blobs {
        self.blobs
            .clone()
            .values()
            .map(|v| v.clone().into())
            .collect_vec()
    }

    pub fn summary(&self, network_id: NetworkID) -> ManifestSummary {
        let ret_summary = RET_summary(&self.secret_magic.scrypto_manifest);
        ManifestSummary::from_ret(ret_summary, network_id)
    }

    pub fn execution_summary(
        &self,
        network_id: NetworkID,
        encoded_receipt: BagOfBytes, // TODO: Replace with TYPE - read from GW.
    ) -> Result<ExecutionSummary> {
        let receipt: TransactionReceipt = encoded_receipt.try_into()?;
        let ret_execution_summary = RET_execution_summary(
            &self.secret_magic.scrypto_manifest,
            &receipt.0,
        )
        .map_err(|e| {
            error!("Failed to get execution summary from RET, error: {:?}", e);
            CommonError::FailedToGetRetExecutionSummaryFromManifest
        })?;

        ExecutionSummary::from_ret(ret_execution_summary, network_id)
    }

    pub fn resource_addresses_to_refresh(
        &self,
    ) -> Option<Vec<ResourceAddress>> {
        todo!()
    }
}

#[uniffi::export]
pub fn new_transaction_manifest_placeholder() -> Manifest {
    Manifest::placeholder_simulator()
}

#[uniffi::export]
pub fn new_transaction_manifest_placeholder_other() -> Manifest {
    Manifest::placeholder_simulator_other()
}

impl HasPlaceholder for Manifest {
    fn placeholder() -> Self {
        Self::placeholder_simulator()
    }

    fn placeholder_other() -> Self {
        Self::placeholder_simulator_other()
    }
}

impl Manifest {
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

    impl FromStr for Manifest {
        type Err = crate::CommonError;

        fn from_str(s: &str) -> Result<Self> {
            Self::new(s.to_owned(), NetworkID::Simulator, Vec::new())
        }
    }

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Manifest;

    #[test]
    fn placeholder_string_roundtrip() {
        let sut = SUT::placeholder();
        assert_eq!(sut.clone(), sut.clone());
        assert_eq!(SUT::placeholder_simulator_instructions_string(), sut.clone().instructions_string());
        assert_eq!(sut.secret_magic.instructions.len(), 3);
    }

    #[test]
    fn multi_account_resource_transfer() {
        let sut = SUT::placeholder_other();
        assert_eq!(sut.clone(), sut.clone());
        assert_eq!(SUT::placeholder_other_simulator_instructions_string(), sut.clone().instructions_string());
        assert_eq!(sut.secret_magic.instructions.len(), 8);
    }
}
