use crate::prelude::*;
use sargon::SessionID as InternalSessionID;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct SessionID {
    pub value: Uuid,
}

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