use crate::prelude::*;

dummy_sargon!(TransactionIntent);

#[allow(unused_variables)]
impl TransactionIntent {
    pub fn new(
        header: TransactionHeader,
        manifest: TransactionManifest,
        message: Message,
    ) -> Self {
        todo!()
    }

    pub fn intent_hash(&self) -> TransactionHash {
        todo!()
    }

    pub fn compile(&self) -> Result<BagOfBytes> {
        todo!()
    }
}
