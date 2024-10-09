use crate::prelude::*;
use sargon::DappOrigin as InternalDappOrigin;

/// The origin of a dapp.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
     uniffi::Record,
)]
pub struct DappOrigin {
    pub value: String,
}

impl From<InternalDappOrigin> for DappOrigin {
    fn from(value: InternalDappOrigin) -> Self {
        Self {
            value: value.0,
        }
    }
}

impl Into<InternalDappOrigin> for DappOrigin {
    fn into(self) -> InternalDappOrigin {
        InternalDappOrigin(self.value)
    }
}