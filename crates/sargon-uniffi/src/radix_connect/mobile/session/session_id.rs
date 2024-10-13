use crate::prelude::*;
use sargon::SessionID as InternalSessionID;

uniffi::custom_newtype!(SessionID, Uuid);

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2)]
pub struct SessionID(pub Uuid);

delegate_display_debug_into!(SessionID, InternalSessionID);
