use crate::prelude::*;
use sargon::InstructionsV2 as InternalInstructionsV2;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct InstructionsV2 {
    pub secret_magic: BagOfBytes, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl InstructionsV2 {
    pub fn into_internal(&self) -> InternalInstructionsV2 {
        self.clone().into()
    }
}

impl From<InternalInstructionsV2> for InstructionsV2 {
    fn from(value: InternalInstructionsV2) -> Self {
        Self {
            secret_magic: value.instructions_as_bytes().into(),
            network_id: value.network_id.into(),
        }
    }
}

impl Into<InternalInstructionsV2> for InstructionsV2 {
    fn into(self) -> InternalInstructionsV2 {
        InternalInstructionsV2::new_from_byte_instructions(
            self.secret_magic.to_vec(),
            self.network_id.into_internal(),
        )
        .unwrap()
    }
}

decl_conversion_tests_for!(InstructionsV2);
