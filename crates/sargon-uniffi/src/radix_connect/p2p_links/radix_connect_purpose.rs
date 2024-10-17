use crate::prelude::*;
use sargon::RadixConnectPurpose as InternalRadixConnectPurpose;

json_string_convertible!(RadixConnectPurpose);

/// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
/// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
/// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum RadixConnectPurpose {
    General,
    Unknown,
}

#[uniffi::export]
pub fn new_radix_connect_purpose_from_string(
    string: String,
) -> RadixConnectPurpose {
    InternalRadixConnectPurpose::from_str_default_value(&string).into()
}

#[uniffi::export]
pub fn new_radix_connect_purpose_sample() -> RadixConnectPurpose {
    InternalRadixConnectPurpose::sample().into()
}

#[uniffi::export]
pub fn new_radix_connect_purpose_sample_other() -> RadixConnectPurpose {
    InternalRadixConnectPurpose::sample_other().into()
}
