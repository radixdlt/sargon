use crate::prelude::*;
use sargon::LinkConnectionQRData as InternalLinkConnectionQRData;

/// The QR code data scanned from the Connector Extension
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::Debug,
    uniffi::Record,
)]
#[debug(
    "LinkConnectionQRData {{ purpose: '{purpose}', password: '{password}', public_key_of_other_party: '{public_key_of_other_party}', signature: '{signature}' }}"
)]
#[display("{}", self.to_obfuscated_string())]
pub struct LinkConnectionQRData {
    /// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
    /// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
    /// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
    pub purpose: RadixConnectPurpose,

    /// Used to be able to re-establish the P2P connection
    pub password: RadixConnectPassword,

    /// Each client generates a curve25119 keypair. The public key will be used as an identifier for the client.
    /// Each client keeps a record of linked clients' public keys to prevent duplicate links.
    /// This is the public key of the other client and it also serves as the seed for the link `ID`.
    pub public_key_of_other_party: Ed25519PublicKey,

    /// Represents a signature produced by Connector Extension by signing the hash of the `password`
    /// with the private key of the `public_key_of_other_party`.
    pub signature: Ed25519Signature,
}

impl From<InternalLinkConnectionQRData> for LinkConnectionQRData {
    fn from(value: InternalLinkConnectionQRData) -> Self {
        Self {
            purpose: value.purpose.into(),
            password: value.password.into(),
            public_key_of_other_party: value.public_key_of_other_party.into(),
            signature: value.signature.into(),
        }
    }
}

impl Into<InternalLinkConnectionQRData> for LinkConnectionQRData {
    fn into(self) -> InternalLinkConnectionQRData {
        InternalLinkConnectionQRData {
            purpose: self.purpose.into(),
            password: self.password.into(),
            public_key_of_other_party: self.public_key_of_other_party.into(),
            signature: self.signature.into(),
        }
    }
}


json_data_convertible!(LinkConnectionQRData);

#[uniffi::export]
pub fn new_link_connection_qr_data_sample() -> LinkConnectionQRData {
    InternalLinkConnectionQRData::sample().into()
}

#[uniffi::export]
pub fn new_link_connection_qr_data_sample_other() -> LinkConnectionQRData {
    InternalLinkConnectionQRData::sample_other().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LinkConnectionQRData;

    #[test]
    fn sample_values() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_link_connection_qr_data_sample(),
                new_link_connection_qr_data_sample_other(),
                // duplicates should get removed
                new_link_connection_qr_data_sample(),
                new_link_connection_qr_data_sample_other(),
            ])
            .len(),
            2
        );
    }
}
