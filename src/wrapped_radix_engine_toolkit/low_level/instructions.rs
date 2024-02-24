use std::ops::Deref;

use crate::prelude::*;

use radix_engine_toolkit::functions::instructions::{
    compile as RET_compile_instructions,
    decompile as RET_decompile_instructions,
};
use transaction::{
    manifest::compile as scrypto_compile,
    manifest::decompile as scrypto_decompile,
    manifest::MockBlobProvider as ScryptoMockBlobProvider,
    prelude::InstructionV1 as ScryptoInstruction,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionsInner(pub(crate) Vec<ScryptoInstruction>);

impl crate::UniffiCustomTypeConverter for InstructionsInner {
    type Builtin = BagOfBytes;

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let bytes: &[u8] = val.bytes();
        RET_decompile_instructions(bytes)
            .map_err(|e| {
                let err_msg = format!("{:?}", e);
                error!("{}", err_msg);
                CommonError::Unknown.into()
            })
            .map(|i: Vec<ScryptoInstruction>| Self(i))
    }

    #[cfg(not(tarpaulin_include))] // false negative, tested in bindgen tests
    fn from_custom(obj: Self) -> Self::Builtin {
        RET_compile_instructions(&obj.0)
            .map(|b| b.into())
            .expect("to never fail")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct Instructions {
    pub(crate) secret_magic: InstructionsInner,
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
            secret_magic: InstructionsInner(manifest.instructions),
            network_id,
        })
    }
}
