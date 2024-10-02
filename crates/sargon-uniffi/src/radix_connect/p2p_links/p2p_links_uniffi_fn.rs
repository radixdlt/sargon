use crate::prelude::*;

#[uniffi::export]
pub fn new_p2p_links_from_json_bytes(
    json_bytes: &BagOfBytes,
) -> Result<P2PLinks> {
    P2PLinks::new_from_json_bytes(json_bytes)
}

#[uniffi::export]
pub fn p2p_links_to_json_bytes(links: P2PLinks) -> BagOfBytes {
    BagOfBytes::from(links.to_json_bytes())
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
