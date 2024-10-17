use crate::prelude::*;
use sargon::DappOrigin as InternalDappOrigin;

uniffi::custom_newtype!(DappOrigin, String);

/// The origin of a dapp.
#[derive(Clone, PartialEq, Eq, InternalConversion)]
pub struct DappOrigin(pub String);
