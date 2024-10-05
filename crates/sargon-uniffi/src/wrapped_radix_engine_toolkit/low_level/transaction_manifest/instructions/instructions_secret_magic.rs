use crate::prelude::*;
use sargon::InstructionsSecretMagic as InternalInstructionsSecretMagic;

/// An internal representation of a collection of Instructions,
/// which intentions is to allow the `struct Instructions`
/// to have no public initializers in Swift/Kotlin land, since it
/// can contain a field:
/// `private let secretMagic: InstructionsSecretMagic`
/// And hide its initializers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstructionsSecretMagic(pub Vec<ScryptoInstruction>);

impl From<InternalInstructionsSecretMagic> for InstructionsSecretMagic {
    fn from(value: InternalInstructionsSecretMagic) -> Self {
        Self(value.0)
    }
}

impl Into<InternalInstructionsSecretMagic> for InstructionsSecretMagic {
    fn into(self) -> InternalInstructionsSecretMagic {
        InternalInstructionsSecretMagic(self.0)
    }
}

uniffi::custom_type!(InstructionsSecretMagic, BagOfBytes);

impl crate::UniffiCustomTypeConverter for InstructionsSecretMagic {
    type Builtin = BagOfBytes;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        let bytes: &[u8] = val.bytes();
        RET_decompile_instructions(bytes)
            .map_err(|e| {
                let err_msg = format!("{:?}", e);
                error!("{}", err_msg);
                CommonError::FailedToUniFFIDecodeBytesToManifestInstructions
                    .into()
            })
            .map(Self::new)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        RET_compile_instructions(&obj.0)
            .map(|b| b.into())
            .expect("to never fail")
    }
}
