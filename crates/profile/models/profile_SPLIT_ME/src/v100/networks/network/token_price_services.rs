use crate::prelude::*;

decl_identified_vec_of!(
    /// Ordered token price service endpoints used for failover.
    TokenPriceServices,
    TokenPriceService
);

impl TokenPriceServices {
    /// "Default" for this collection in profile/network context:
    /// one production endpoint.
    pub fn default() -> Self {
        Self::just(TokenPriceService::production())
    }

    pub fn is_default(&self) -> bool {
        self == &Self::default()
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
    fn json_roundtrip() {
        let sut = SUT::sample_other();
        assert_json_roundtrip(&sut);
    }
}
