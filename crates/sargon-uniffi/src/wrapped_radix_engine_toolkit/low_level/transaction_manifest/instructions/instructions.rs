use crate::prelude::*;
use sargon::Instructions as InternalInstructions;

#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display, uniffi::Record)]
#[display("{}", self.instructions_string())]
pub struct Instructions {
    pub secret_magic: InstructionsSecretMagic, // MUST be first prop, else you break build.
    pub network_id: NetworkID,
}

impl From<InternalInstructions> for Instructions {
    fn from(value: InternalInstructions) -> Self {
        Self {
            secret_magic: value.secret_magic.into(),
            network_id: value.network_id.into(),
        }
    }
}

impl Into<InternalInstructions> for Instructions {
    fn into(self) -> InternalInstructions {
        InternalInstructions {
            secret_magic: self.secret_magic.into(),
            network_id: self.network_id.into(),
        }
    }
}