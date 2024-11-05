use crate::prelude::*;
use sargon::AddressFormat as InternalAddressFormat;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct FormattedAddress {
    pub full: String,
    pub raw: String,
    pub default: String,
}
