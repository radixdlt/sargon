use crate::prelude::*;
use sargon::DappOrigin as InternalDappOrigin;

uniffi::custom_newtype!(DappOrigin, String);

/// The origin of a dapp.
#[derive(
    Clone,
    PartialEq,
    Eq,
    Debug,
    derive_more::Display,
)]
pub struct DappOrigin(pub(crate) String);

impl From<InternalDappOrigin> for DappOrigin {
    fn from(value: InternalDappOrigin) -> Self {
        Self(value.0))
    }
}

impl Into<InternalDappOrigin> for DappOrigin {
    fn into(self) -> InternalDappOrigin {
        InternalDappOrigin(self.0)
    }
}