use crate::prelude::*;
use sargon::RadixConnectPurpose as InternalRadixConnectPurpose;

json_string_convertible!(RadixConnectPurpose);

/// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
/// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
/// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    uniffi::Enum,
)]
pub enum RadixConnectPurpose {
    General,
    Unknown,
}

impl From<InternalRadixConnectPurpose> for RadixConnectPurpose {
    fn from(value: InternalRadixConnectPurpose) -> Self {
        match value {
            InternalRadixConnectPurpose::General => Self::General,
            InternalRadixConnectPurpose::Unknown => Self::Unknown,
        }
    }
}

impl Into<InternalRadixConnectPurpose> for RadixConnectPurpose {
    fn into(self) -> InternalRadixConnectPurpose {
        match self {
            Self::General => InternalRadixConnectPurpose::General,
            Self::Unknown => InternalRadixConnectPurpose::Unknown,
        }
    }
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

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn new() {
        assert_eq!(
            new_radix_connect_purpose_from_string(String::from("general")),
            RadixConnectPurpose::General
        );
    }

    #[test]
    fn sample_values() {
        assert_eq!(
            new_radix_connect_purpose_sample(),
            RadixConnectPurpose::General
        );
        assert_eq!(
            new_radix_connect_purpose_sample_other(),
            RadixConnectPurpose::Unknown
        );
    }
}
