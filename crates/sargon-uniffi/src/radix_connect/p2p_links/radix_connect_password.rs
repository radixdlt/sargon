use crate::prelude::*;
use sargon::RadixConnectPassword as InternalRadixConnectPassword;

json_string_convertible!(RadixConnectPassword);

/// The hash of the connection password is used to connect to the Radix Connect Signaling Server,
/// over web sockets. The actual `ConnectionPassword` is used to encrypt all messages sent via
/// the Signaling Server.
#[derive(
    Clone,
    
    PartialEq,
    Eq,
    Hash,
    InternalConersion,
     uniffi::Record,
)]
pub struct RadixConnectPassword {
    pub value: Exactly32Bytes,
}

impl From<InternalRadixConnectPassword> for RadixConnectPassword {
    fn from(value: InternalRadixConnectPassword) -> Self {
        Self { value: value.value.into() }
    }
}

impl Into<InternalRadixConnectPassword> for RadixConnectPassword {
    fn into(self) -> InternalRadixConnectPassword {
        InternalRadixConnectPassword::new(self.value.into())
    }
}

#[uniffi::export]
pub fn new_radix_connect_password(
    bytes: Exactly32Bytes,
) -> RadixConnectPassword {
    InternalRadixConnectPassword::new(bytes.into()).into()
}

#[uniffi::export]
pub fn new_radix_connect_password_sample() -> RadixConnectPassword {
    InternalRadixConnectPassword::sample().into()
}

#[uniffi::export]
pub fn new_radix_connect_password_sample_other() -> RadixConnectPassword {
    InternalRadixConnectPassword::sample_other().into()
}

#[uniffi::export]
pub fn radix_connect_password_message_hash(
    password: &RadixConnectPassword,
) -> Hash {
    password.into_internal().message_hash().into()
}

