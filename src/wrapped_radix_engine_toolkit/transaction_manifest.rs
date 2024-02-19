use crate::prelude::*;

dummy_sargon!(TransactionManifest);

#[allow(unused_variables)]
impl TransactionManifest {
    pub fn new(
        instructions_string: String,
        network_id: NetworkID,
        blobs: Blobs,
    ) -> Self {
        todo!()
    }

    pub fn resource_addresses_to_refresh(
        &self,
    ) -> Option<Vec<ResourceAddress>> {
        todo!()
    }

    pub fn instructions_string(&self) -> String {
        todo!()
    }

    pub fn blobs(&self) -> Blobs {
        todo!()
    }

    pub fn summary(&self, network_id: NetworkID) -> ManifestSummary {
        todo!()
    }

    pub fn execution_summary(
        &self,
        network_id: NetworkID,
        encoded_receipt: BagOfBytes, // TODO: Replace with TYPE - read from GW.
    ) -> ExecutionSummary {
        todo!()
    }
}
