use crate::prelude::*;

decl_identified_vec_of!(
    /// Ordered token price service endpoints used for failover.
    TokenPriceServices,
    TokenPriceService
);

impl TokenPriceServices {
    /// "Default" for this collection in profile/network context:
    /// one production endpoint.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Self {
        Self::just(TokenPriceService::production())
    }

    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }

    /// The same services with the former production service replaced, in
    /// its position, by the current one. When the current one is already
    /// listed, the earlier of the two positions is kept.
    pub fn replacing_legacy_production(self) -> Self {
        let legacy = TokenPriceService::legacy_production();
        Self::from_iter(self.into_iter().map(|service| {
            if service == legacy {
                TokenPriceService::production()
            } else {
                service
            }
        }))
    }

    /// Reads services stored in a Profile. A list is stored only once the
    /// user adds a service, and then still holds the default of that time,
    /// so the former production service is swapped for the current one.
    pub fn deserialize_replacing_legacy_production<'de, D>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Self::deserialize(deserializer).map(Self::replacing_legacy_production)
    }

    pub fn add(&mut self, service: TokenPriceService) -> bool {
        self.try_insert_unique(service).is_ok()
    }

    pub fn remove_by_base_url(&mut self, base_url: &Url) -> bool {
        if self.len() <= 1 {
            return false;
        }
        self.remove_id(base_url).is_some()
    }
}

impl HasSampleValues for TokenPriceServices {
    fn sample() -> Self {
        Self::default()
    }

    fn sample_other() -> Self {
        Self::from_iter([
            TokenPriceService::sample(),
            TokenPriceService::sample_other(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = TokenPriceServices;

    #[test]
    fn add_duplicate_returns_false() {
        let mut sut = SUT::new();
        let service = TokenPriceService::sample();

        assert!(sut.add(service.clone()));
        assert!(!sut.add(service));
    }

    #[test]
    fn insertion_order_is_preserved() {
        let mut sut = SUT::new();
        let first = TokenPriceService::sample();
        let second = TokenPriceService::sample_other();

        assert!(sut.add(first.clone()));
        assert!(sut.add(second.clone()));

        let all = sut.get_all().into_iter().cloned().collect_vec();
        assert_eq!(all, vec![first, second]);
    }

    #[test]
    fn remove_missing_returns_false() {
        let mut sut = SUT::sample();
        let missing = Url::parse("https://missing.example").unwrap();
        assert!(!sut.remove_by_base_url(&missing));
    }

    #[test]
    fn remove_last_is_disallowed() {
        let mut sut = SUT::sample();
        let only = sut.get_all().first().unwrap().base_url.clone();
        assert!(!sut.remove_by_base_url(&only));
        assert_eq!(sut.len(), 1);
    }

    #[test]
    fn remove_existing_when_more_than_one_returns_true() {
        let mut sut = SUT::sample_other();
        let to_remove = TokenPriceService::sample_other().base_url;
        assert!(sut.remove_by_base_url(&to_remove));
        assert_eq!(sut.len(), 1);
    }

    #[test]
    fn remove_production_when_more_than_one_returns_true() {
        let mut sut = SUT::sample_other();
        let production = TokenPriceService::production().base_url;
        assert!(sut.remove_by_base_url(&production));
        assert_eq!(sut, SUT::just(TokenPriceService::sample_other()));
    }

    fn service(url: &str) -> TokenPriceService {
        TokenPriceService::new(Url::parse(url).unwrap())
    }

    fn urls(sut: &SUT) -> Vec<String> {
        sut.iter().map(|s| s.base_url.to_string()).collect_vec()
    }

    #[test]
    fn replacing_legacy_production_keeps_its_position() {
        let stored = SUT::from_iter([
            service("https://custom.example"),
            service("https://token-price-service.radixdlt.com"),
            service("https://other.example"),
        ]);
        assert_eq!(
            urls(&stored.replacing_legacy_production()),
            vec![
                "https://custom.example/",
                "https://prices.radixdao.org/",
                "https://other.example/",
            ]
        );
    }

    #[test]
    fn replacing_legacy_production_when_current_is_listed_keeps_earlier() {
        let stored = SUT::from_iter([
            service("https://token-price-service.radixdlt.com"),
            service("https://custom.example"),
            TokenPriceService::production(),
        ]);
        assert_eq!(
            urls(&stored.replacing_legacy_production()),
            vec!["https://prices.radixdao.org/", "https://custom.example/"]
        );
    }

    #[test]
    fn replacing_legacy_production_leaves_lists_without_it() {
        let stored = SUT::just(service("https://custom.example"));
        assert_eq!(stored.clone().replacing_legacy_production(), stored);
        assert_eq!(
            SUT::default().replacing_legacy_production(),
            SUT::default()
        );
    }

    #[test]
    fn json_roundtrip() {
        let sut = SUT::sample_other();
        assert_json_roundtrip(&sut);
    }
}
