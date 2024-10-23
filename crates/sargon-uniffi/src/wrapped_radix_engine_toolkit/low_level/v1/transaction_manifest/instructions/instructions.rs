use crate::prelude::*;
use sargon::Instructions as InternalInstructions;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct Instructions {
    pub secret_magic: String,
    pub network_id: NetworkID,
}

impl Instructions {
    pub fn into_internal(&self) -> InternalInstructions {
        self.clone().into()
    }
}

impl From<InternalInstructions> for Instructions {
    fn from(value: InternalInstructions) -> Self {
        Self {
            secret_magic: value.instructions_string(),
            network_id: value.network_id.into(),
        }
    }
}

impl Into<InternalInstructions> for Instructions {
    fn into(self) -> InternalInstructions {
        InternalInstructions::new(self.secret_magic, self.network_id.into())
            .unwrap()
    }
}

decl_conversion_tests_for!(Instructions);
