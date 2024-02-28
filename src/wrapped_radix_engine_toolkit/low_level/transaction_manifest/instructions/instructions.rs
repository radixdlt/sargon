use std::ops::Deref;

use crate::prelude::*;

use transaction::{
    manifest::compile as scrypto_compile,
    manifest::decompile as scrypto_decompile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::InstructionV1 as ScryptoInstruction,
};

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Instructions {
    pub(crate) secret_magic: InstructionsSecretMagic, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl Deref for Instructions {
    type Target = Vec<ScryptoInstruction>;

    fn deref(&self) -> &Self::Target {
        &self.secret_magic.0
    }
}

impl Instructions {
    pub fn instructions_string(&self) -> String {
        let network_definition = self.network_id.network_definition();
        scrypto_decompile(self, &network_definition).expect("Should never fail, because should never have allowed invalid instructions")
    }

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
        .map(|manifest| Self {
            secret_magic: InstructionsSecretMagic(manifest.instructions),
            network_id,
        })
    }
}
