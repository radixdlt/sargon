use crate::prelude::*;
use sargon::LinkConnectionQRData as InternalLinkConnectionQRData;

/// The QR code data scanned from the Connector Extension
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
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

json_data_convertible!(LinkConnectionQRData);

#[uniffi::export]
pub fn new_link_connection_qr_data_sample() -> LinkConnectionQRData {
    InternalLinkConnectionQRData::sample().into()
}

#[uniffi::export]
pub fn new_link_connection_qr_data_sample_other() -> LinkConnectionQRData {
    InternalLinkConnectionQRData::sample_other().into()
}
