use crate::prelude::*;
use sargon::P2PLinks as InternalP2PLinks;

#[uniffi::export]
pub fn new_p2p_links_from_json_bytes(
    json_bytes: &BagOfBytes,
) -> Result<Vec<P2PLink>> {
    json_bytes
        .to_vec()
        .deserialize::<InternalP2PLinks>()
        .into_iter_result()
}

#[uniffi::export]
pub fn p2p_links_to_json_bytes(p2p_links: Vec<P2PLink>) -> BagOfBytes {
    let internal: InternalP2PLinks = p2p_links.into_internal();
    let bytes = internal.serialize_to_bytes().unwrap();
    bytes.into()
}
