use identified_vec::Identifiable;

use serde::{Deserialize, Serialize};

use super::p2p_link::P2PLink;

#[cfg(any(test, feature = "placeholder"))]
use crate::HasPlaceholder;

// pub type P2PLinks = IdentifiedVecVia<P2PLink>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
#[serde(transparent)]
pub struct P2PLinks {
    // FIXME: Now
    vec: Vec<P2PLink>,
}

impl P2PLinks {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = P2PLink>,
    {
        Self {
            vec: Vec::from_iter(iter.into_iter()),
        }
    }

    pub fn append(&mut self, link: P2PLink) {
        if self.vec.iter().any(|x| x.id() == link.id()) {
            return;
        }
        self.vec.push(link);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

impl Default for P2PLinks {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(any(test, feature = "placeholder"))]
impl HasPlaceholder for P2PLinks {
    /// A placeholder used to facilitate unit tests.
    fn placeholder() -> Self {
        Self::from_iter([P2PLink::placeholder_brave(), P2PLink::placeholder_chrome()])
    }

    /// A placeholder used to facilitate unit tests.
    fn placeholder_other() -> Self {
        Self::from_iter([
            P2PLink::placeholder_arc(),
            P2PLink::placeholder_duckduckgo(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_eq_after_json_roundtrip, HasPlaceholder};

    use super::P2PLinks;

    #[test]
    fn equality() {
        assert_eq!(P2PLinks::placeholder(), P2PLinks::placeholder());
        assert_eq!(P2PLinks::placeholder_other(), P2PLinks::placeholder_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(P2PLinks::placeholder(), P2PLinks::placeholder_other());
    }
    // #[test]
    // fn display() {
    //     let mut sut = P2PLinks::new();
    //     sut.append(P2PLink::placeholder_duckduckgo());
    //     assert_eq!(format!("{}", sut), "[P2PLink { connection_password: RadixConnectPassword(deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead), display_name: \"DuckDuckGo on Mac Pro\" }]");
    // }

    // #[test]
    // fn into_iter() {
    //     let mut sut = P2PLinks::new();
    //     sut.append(P2PLink::placeholder_duckduckgo());
    //     sut.append(P2PLink::placeholder_chrome());
    //     assert!(sut.into_iter().any(|p| p.display_name().contains("Chrome")));
    // }

    #[test]
    fn default_is_empty() {
        assert_eq!(P2PLinks::default().len(), 0);
    }

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

    // #[test]
    // fn duplicates_are_not_allowed() {
    //     let mut sut =
    //         P2PLinks::from_iter([P2PLink::placeholder_brave(), P2PLink::placeholder_chrome()]);
    //     let (inserted, _) = sut.append(P2PLink::placeholder_brave());
    //     assert_eq!(inserted, false);
    // }

    // #[test]
    // fn order_is_maintained() {
    //     let a = P2PLink::placeholder_arc();
    //     let b = P2PLink::placeholder_brave();
    //     let c = P2PLink::placeholder_chrome();
    //     let d = P2PLink::placeholder_duckduckgo();
    //     let mut sut = P2PLinks::from_iter([&a, &b, &c].into_iter().cloned());
    //     assert_eq!(sut.elements(), [&a, &b, &c]);
    //     sut.insert(d.clone(), 1);
    //     assert_eq!(sut.elements(), [&a, &d, &b, &c]);
    // }
}
