use crate::prelude::*;
use sargon::P2PLinks as InternalP2PLinks;
use sargon::BagOfBytes as InternalBagOfBytes;

decl_identified_vec_of!(
    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with the dApp or Connector Extension
    P2PLink
);

impl JsonDataDeserializing for P2PLinks {}
impl JsonDataSerializing for P2PLinks {}

use crate::prelude::*;

#[uniffi::export]
pub fn new_p2p_links_from_json_bytes(
    json_bytes: &BagOfBytes,
) -> Result<P2PLinks> {
    map_result_from_internal(InternalP2PLinks::new_from_json_bytes(&json_bytes.into()))
}

#[uniffi::export]
pub fn p2p_links_to_json_bytes(links: P2PLinks) -> BagOfBytes {
    InternalBagOfBytes::from(links.into::<InternalP2PLinks>().to_json_bytes()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = P2PLinks;

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample();
        let json = p2p_links_to_json_bytes(sut.clone());
        let deserialized = new_p2p_links_from_json_bytes(&json).unwrap();
        assert_eq!(deserialized, sut)
    }
}

