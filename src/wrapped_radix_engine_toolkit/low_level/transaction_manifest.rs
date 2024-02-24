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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instructions(pub(crate) Vec<ScryptoInstruction>, NetworkID);

impl Instructions {
    pub fn new(
        instructions_string: String,
        network_id: NetworkID,
    ) -> Result<Self> {
        let network_definition = network_id.network_definition();
        let blob_provider = ScryptoMockBlobProvider::new();
        scrypto_compile(
            &instructions_string,
            &network_definition,
            blob_provider,
        )
        .map_err(|_e| CommonError::InvalidInstructionsString)
        .map(|manifest| Self(manifest.instructions, network_id))
    }
}

impl Manifest {
    pub fn new(
        instructions_string: String,
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

#[derive(Clone, Debug)]
pub struct TransactionReceipt(pub(crate) ScryptoTransactionReceipt);
impl TryFrom<Vec<u8>> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        scrypto_decode(&value).map_err(|e| {
            error!("Failed to decode encoded Transaction Receipt (bytes) into a (Scrypto)TransactionReceipt, error: {:?}", e);
            CommonError::FailedToDecodeEncodedReceipt
        }).map(Self)
    }
}
impl TryFrom<BagOfBytes> for TransactionReceipt {
    type Error = crate::CommonError;

    fn try_from(value: BagOfBytes) -> Result<Self, Self::Error> {
        Self::try_from(value.to_vec())
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

    // https://github.com/radixdlt/radix-engine-toolkit/blob/cf2f4b4d6de56233872e11959861fbf12db8ddf6/crates/radix-engine-toolkit/tests/manifests/account/resource_transfer.rtm
    #[test]
    fn resource_transfer() {
        let manifest_str = r#"
CALL_METHOD
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
"#;

        let manifest: Manifest = manifest_str.parse().unwrap();

        assert_eq!(manifest.clone(), manifest.clone());
        assert_eq!(
            manifest.clone().instructions_string().trim(),
            manifest_str.trim()
        );
        assert_eq!(manifest.secret_magic.instructions.len(), 3);
    }
}
