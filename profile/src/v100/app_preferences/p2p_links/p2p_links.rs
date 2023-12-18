use super::p2p_link::P2PLink;
use identified_vec::{newtype_identified_vec, IsIdentifiedVecOf};

newtype_identified_vec!(of: P2PLink, named: P2PLinks);

impl Default for P2PLinks {
    fn default() -> Self {
        Self::new()
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
    use identified_vec::{IsIdentifiedVec, IsIdentifiedVecOf};
    use wallet_kit_common::json::assert_eq_after_json_roundtrip;

    use crate::v100::app_preferences::p2p_links::p2p_link::P2PLink;

    use super::P2PLinks;

    #[test]
    fn json_roundtrip() {
        let sut = P2PLinks::placeholder();
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            [
                {
                    "connectionPassword": "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe",
                    "displayName": "Brave on PC"
                },
                {
                    "connectionPassword": "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe",
                    "displayName": "Chrome on Macbook"
                }
            ]
            "#,
        )
    }

    #[test]
    fn duplicates_are_not_allowed() {
        let mut sut =
            P2PLinks::from_iter([P2PLink::placeholder_brave(), P2PLink::placeholder_chrome()]);
        let (inserted, _) = sut.append(P2PLink::placeholder_brave());
        assert_eq!(inserted, false);
    }

    #[test]
    fn order_is_maintained() {
        let a = P2PLink::placeholder_arc();
        let b = P2PLink::placeholder_brave();
        let c = P2PLink::placeholder_chrome();
        let d = P2PLink::placeholder_duckduckgo();
        let mut sut = P2PLinks::from_iter([&a, &b, &c].into_iter().cloned());
        assert_eq!(sut.elements(), [&a, &b, &c]);
        sut.insert(d.clone(), 1);
        assert_eq!(sut.elements(), [&a, &d, &b, &c]);
    }
}
