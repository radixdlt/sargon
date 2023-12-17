use super::p2p_link::P2PLink;
use identified_vec::{newtype_identified_vec, IsIdentifiedVecOf};

newtype_identified_vec!(of: P2PLink, named: P2PLinks);

impl P2PLinks {
    /// A placeholder used to facilitate unit tests.
    pub fn placeholder() -> Self {
        Self::from_iter([P2PLink::placeholder_brave(), P2PLink::placeholder_chrome()])
    }
}

#[cfg(test)]
mod tests {
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use super::P2PLinks;

    #[test]
    fn json_roundtrip() {
        let sut = P2PLinks::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "connectionPassword": "fadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaffadedeaf",
                    "displayName": "Brave on PC"
                },
                {
                    "connectionPassword": "deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
                    "displayName": "Chrome on Macbook"
                }
            ]
            "#,
        )
    }

    // #[test]
    // fn duplicates_are_not_allowed() {
    //     P2PLinks::
    // }
}
