use crate::prelude::*;

use transaction::{
    manifest::compile as scrypto_compile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::InstructionV1 as ScryptoInstruction,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instructions(pub(crate) Vec<ScryptoInstruction>, NetworkID);

impl Instructions {
    pub fn new(
        instructions_string: impl AsRef<str>,
        network_id: NetworkID,
    ) -> Result<Self> {
        let network_definition = network_id.network_definition();
        let blob_provider = ScryptoMockBlobProvider::new();
        scrypto_compile(
            instructions_string.as_ref(),
            &network_definition,
            blob_provider,
        )
        .map_err(|_e| CommonError::InvalidInstructionsString)
        .map(|manifest| Self(manifest.instructions, network_id))
    }
}
