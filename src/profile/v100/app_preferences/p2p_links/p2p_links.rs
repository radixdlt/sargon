use crate::prelude::*;

decl_ordered_map!(
    /// Collection of clients user have connected P2P with, typically these
    /// are WebRTC connections with DApps, but might be Android or iPhone
    /// clients as well.
    P2PLink
);

impl HasSampleValues for P2PLinks {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_iter([P2PLink::sample_brave(), P2PLink::sample_chrome()])
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::from_iter([P2PLink::sample_arc(), P2PLink::sample_duckduckgo()])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn equality() {
        assert_eq!(P2PLinks::sample(), P2PLinks::sample());
        assert_eq!(P2PLinks::sample_other(), P2PLinks::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(P2PLinks::sample(), P2PLinks::sample_other());
    }

    #[test]
    fn into_iter() {
        let mut sut = P2PLinks::new();
        sut.append(P2PLink::sample_duckduckgo());
        sut.append(P2PLink::sample_chrome());
        assert!(sut.into_iter().any(|p| p.display_name.contains("Chrome")));
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(P2PLinks::default().len(), 0);
    }

    #[test]
    fn json_roundtrip() {
        let sut = P2PLinks::sample();
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
        let mut sut = P2PLinks::from_iter([
            P2PLink::sample_brave(),
            P2PLink::sample_chrome(),
        ]);
        let (inserted, _) = sut.append(P2PLink::sample_brave());
        assert!(!inserted);
    }

    #[test]
    fn order_is_maintained() {
        let a = P2PLink::sample_arc();
        let b = P2PLink::sample_brave();
        let c = P2PLink::sample_chrome();
        let d = P2PLink::sample_duckduckgo();
        let mut sut = P2PLinks::from_iter([&a, &b, &c].into_iter().cloned());
        assert_eq!(sut.get_all(), [&a, &b, &c]);
        sut.insert_at(d.clone(), 1);
        assert_eq!(sut.get_all(), [&a, &d, &b, &c]);
    }
}
