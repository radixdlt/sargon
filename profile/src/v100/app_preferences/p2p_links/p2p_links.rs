use identified_vec::IdentifiedVecOf;
use serde::{Deserialize, Serialize};

use super::p2p_link::P2PLink;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct P2PLinks(IdentifiedVecOf<P2PLink>);

impl P2PLinks {
    pub fn new(links: IdentifiedVecOf<P2PLink>) -> Self {
        P2PLinks(links)
    }
}

impl FromIterator<P2PLink> for P2PLinks {
    fn from_iter<T: IntoIterator<Item = P2PLink>>(iter: T) -> Self {
        P2PLinks::new(IdentifiedVecOf::<P2PLink>::from_iter(iter))
    }
}

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
}
