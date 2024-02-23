use std::ops::Deref;

use crate::prelude::*;

use radix_engine_toolkit_uniffi::{
    Instructions as RetInstructions, ManifestSummary as RetManifestSummary,
    TransactionManifest as RetTransactionManifest,
};

pub type Blob = BagOfBytes;
pub type Blobs = Vec<Blob>;

#[derive(Clone, PartialEq, Eq, Debug, uniffi::Object)]
pub struct ManifestInner {
    pub ret: Arc<RetTransactionManifest>,
}
impl Deref for ManifestInner {
    type Target = RetTransactionManifest;

    fn deref(&self) -> &Self::Target {
        &self.ret
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

impl From<Arc<RetTransactionManifest>> for Manifest {
    fn from(value: Arc<RetTransactionManifest>) -> Self {
        ManifestInner { ret: value }.into()
    }
}

impl From<Manifest> for Arc<RetTransactionManifest> {
    fn from(value: Manifest) -> Self {
        value.secret_magic.ret.clone()
    }
}

impl Deref for Manifest {
    type Target = RetTransactionManifest;

    fn deref(&self) -> &Self::Target {
        &self.secret_magic
    }
}

#[allow(unused_variables)]
impl Manifest {
    pub fn new(
        instructions_string: String,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Result<Self> {
        RetInstructions::from_string(
            instructions_string,
            network_id.discriminant(),
        )
        .map_err(|e| CommonError::InvalidInstructionsString)
        .map(|i| {
            RetTransactionManifest::new(
                i,
                blobs.into_iter().map(|b| b.to_vec()).collect_vec(),
            )
        })
        .map(|r: Arc<RetTransactionManifest>| r.into())
        .map(|m: Manifest| m)
    }

    pub fn instructions_string(&self) -> String {
        self.instructions.as_str().expect("Should always be able to string representation of a TransactionManifest's instructions.").to_string()
    }

    /// This clones the blobs which might be expensive resource wise.
    pub fn blobs(&self) -> Blobs {
        self.blobs
            .clone()
            .into_iter()
            .map(|v| v.into())
            .collect_vec()
    }

    pub fn summary(&self, network_id: NetworkID) -> ManifestSummary {
        self.secret_magic
            .ret
            .summary(network_id.discriminant())
            .try_into()
            .expect("to always work")
    }

    pub fn execution_summary(
        &self,
        network_id: NetworkID,
        encoded_receipt: BagOfBytes, // TODO: Replace with TYPE - read from GW.
    ) -> ExecutionSummary {
        self.secret_magic
            .ret
            .execution_summary(
                network_id.discriminant(),
                encoded_receipt.to_vec(),
            )
            .expect("to always work")
            .try_into()
            .expect("to always work")
    }

    pub fn resource_addresses_to_refresh(
        &self,
    ) -> Option<Vec<ResourceAddress>> {
        todo!()
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
        let manifest: Manifest = r#"
            CALL_METHOD 
            Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q") 
            "lock_fee"
            Decimal("500");

            # Withdrawing 100 XRD from the account component
            CALL_METHOD 
                Address("account_sim1cyvgx33089ukm2pl97pv4max0x40ruvfy4lt60yvya744cve475w0q") 
                "withdraw"
                Address("resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3")
                Decimal("100");

            # Depositing all of the XRD withdrawn from the account into the other account
            CALL_METHOD
                Address("account_sim1cyzfj6p254jy6lhr237s7pcp8qqz6c8ahq9mn6nkdjxxxat5syrgz9") 
                "try_deposit_batch_or_abort"
                Expression("ENTIRE_WORKTOP")
                None;
        "#.parse().unwrap();

        assert_eq!(manifest.clone(), manifest.clone());
        assert_eq!(
            manifest.secret_magic.instructions.instructions_list().len(),
            3
        );
    }
}
