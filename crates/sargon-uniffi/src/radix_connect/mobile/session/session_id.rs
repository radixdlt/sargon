use crate::prelude::*;
use sargon::SessionID as InternalSessionID;

uniffi::custom_newtype!(SessionID, Uuid);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
)]
pub struct SessionID(pub Uuid);

impl From<InternalSessionID> for SessionID {
    fn from(value: InternalSessionID) -> Self {
        Self(value.0)
    }
}

impl Into<InternalSessionID> for SessionID {
    fn into(self) -> InternalSessionID {
        InternalSessionID(self.0)
    }
}