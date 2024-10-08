use crate::prelude::*;
use sargon::P2PLink as InternalP2PLink;

/// A client the user have connected P2P with, typically a WebRTC connection with the dApp or Connector Extension.
/// Each client generates a curve25119 keypair. The public key is used as an identifier for the client.
/// The hash of the connection password is used to establish the P2P connection.
/// There can be multiple types of links (trusted vs untrusted) differentiated by `RadixConnectPurpose`.
/// Here are the [CAP-36][doc] requirements.
///
/// [doc]: https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3251863610/CAP-36+WebRTC+Clients+Protocol
#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    uniffi::Record,
)]
pub struct P2PLink {
    /// The most important property of this struct, the `RadixConnectPassword`,
    /// is used to be able to re-establish the P2P connection
    pub connection_password: RadixConnectPassword,

    /// The purpose of the connection, set by the other client, typically Connector Extension or dApp.
    /// As part of the initial linking flow, user will be prompted about kind of link they're trying to make.
    /// The user needs to make a conscious decision about general purpose links (because it comes with security risk).
    pub connection_purpose: RadixConnectPurpose,

    /// Each client generates a curve25119 keypair. The public key will be used as an identifier for the client.
    /// Each client keeps a record of linked clients' public keys to prevent duplicate links.
    /// This is the public key of the other client and it also serves as the seed for the link `ID`.
    pub public_key: Ed25519PublicKey,

    /// Client name, e.g. "Chrome on Macbook" or "My work Android" or "My wifes iPhone SE".
    pub display_name: String,
}

impl From<InternalP2PLink> for P2PLink {
    fn from(value: InternalP2PLink) -> Self {
        Self {
            connection_password: value.connection_password.into(),
            connection_purpose: value.connection_purpose.into(),
            public_key: value.public_key.into(),
            display_name: value.display_name,
        }
    }
}

impl Into<InternalP2PLink> for P2PLink {
    fn into(self) -> InternalP2PLink {
        InternalP2PLink {
            connection_password: self.connection_password.into(),
            connection_purpose: self.connection_purpose.into(),
            public_key: self.public_key.into(),
            display_name: self.display_name,
        }
    }
}


json_data_convertible!(P2PLink);

#[uniffi::export]
pub fn p2p_link_id(link: &P2PLink) -> <InternalP2PLink as sargon::Identifiable>::ID {
    link.into_internal().id()
}

#[uniffi::export]
pub fn new_p2p_link_sample() -> P2PLink {
    InternalP2PLink::sample().into()
}

#[uniffi::export]
pub fn new_p2p_link_sample_other() -> P2PLink {
    InternalP2PLink::sample_other().into()
}

#[cfg(test)]
mod uniffi_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = P2PLink;

    #[test]
    fn id_of_link() {
        let sut = SUT::sample();
        assert_eq!(p2p_link_id(&sut), sut.id())
    }

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_p2p_link_sample(),
                new_p2p_link_sample_other(),
                // duplicates should get removed
                new_p2p_link_sample(),
                new_p2p_link_sample_other(),
            ])
            .len(),
            2
        );
    }
}
