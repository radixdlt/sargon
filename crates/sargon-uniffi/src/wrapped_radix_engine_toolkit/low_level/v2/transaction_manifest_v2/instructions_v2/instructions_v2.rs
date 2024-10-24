use crate::prelude::*;
use sargon::InstructionsV2 as InternalInstructionsV2;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct InstructionsV2 {
    pub secret_magic: String,
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
            secret_magic: value.instructions_string(),
            network_id: value.network_id.into(),
        }
    }
}

impl From<InstructionsV2> for InternalInstructionsV2 {
    fn from(val: InstructionsV2) -> Self {
        InternalInstructionsV2::new(val.secret_magic, val.network_id.into())
            .unwrap()
    }
}

decl_conversion_tests_for!(InstructionsV2);
