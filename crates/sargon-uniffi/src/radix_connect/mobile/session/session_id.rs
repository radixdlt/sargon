use crate::prelude::*;
use sargon::SessionID as InternalSessionID;

#[derive(
    
    Clone,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct SessionID {
    pub value: Uuid,
}

delegate_display_debug_into!(SessionID, InternalSessionID);

impl From<InternalSessionID> for SessionID {
    fn from(value: InternalSessionID) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalSessionID> for SessionID {
    fn into(self) -> InternalSessionID {
        InternalSessionID(self.value)
    }
}