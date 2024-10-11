use crate::prelude::*;
use sargon::AddressFormat as InternalAddressFormat;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversionV2, uniffi::Enum)]
pub enum AddressFormat {
    Full,
    Raw,
    Default,
}
