use crate::prelude::*;

decl_identified_vec_of!(
    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with the dApp or Connector Extension
    P2PLink
);

#[uniffi::export]
pub fn new_p2p_links_from_json_bytes(
    json_bytes: &BagOfBytes,
) -> Result<P2PLinks> {
    json_bytes
        .to_vec()
        .deserialize::<InternalP2PLinks>()
        .map_result()
}

#[uniffi::export]
pub fn new_p2p_links_from_to_bytes(p2p_links: P2PLinks) -> BagOfBytes {
    let internal: InternalP2PLinks = p2p_links.into_identified_vec();
    let bytes = internal.serialize_to_bytes().unwrap();
    bytes.into()
}
