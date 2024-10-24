use crate::prelude::*;
use sargon::AddressFormat as InternalAddressFormat;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}
