use crate::prelude::*;
use sargon::Instructions as InternalInstructions;

#[derive(Clone, PartialEq, Eq, uniffi::Record)]
pub struct Instructions {
    pub value: BagOfBytes,
    pub network_id: NetworkID,
}

impl From<InternalInstructions> for Instructions {
    fn from(value: InternalInstructions) -> Self {
        Self {
            value: value.instructions_as_bytes().into(),
            network_id: value.network_id.into(),
        }
    }
}

impl Into<InternalInstructions> for Instructions {
    fn into(self) -> InternalInstructions {
        InternalInstructions::new_from_byte_instructions(
            self.value.to_vec(),
            self.network_id.into_internal(),
        )
        .unwrap()
    }
}
