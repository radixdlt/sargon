use crate::prelude::*;

use radix_engine_toolkit::functions::instructions::{
    compile as RET_compile_instructions,
    decompile as RET_decompile_instructions,
};
use transaction::prelude::InstructionV1 as ScryptoInstruction;

/// An internal representation of a collection of Instructions,
/// which intentions is to allow the `struct Instructions`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a field:
/// `private let secretMagic: InstructionsSecretMagic`
/// And hide its initializers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionsSecretMagic(pub(crate) Vec<ScryptoInstruction>);

impl crate::UniffiCustomTypeConverter for InstructionsSecretMagic {
    type Builtin = BagOfBytes;

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

    fn from_custom(obj: Self) -> Self::Builtin {
        RET_compile_instructions(&obj.0)
            .map(|b| b.into())
            .expect("to never fail")
    }
}
