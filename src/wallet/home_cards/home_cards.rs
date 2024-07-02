use crate::prelude::*;

decl_identified_vec_of!(HomeCard);

impl HasSampleValues for HomeCards {
    fn sample() -> Self {
        Self::from_iter([HomeCard::sample(), HomeCard::Connector])
    }

    fn sample_other() -> Self {
        Self::from_iter([
            HomeCard::sample_other(),
            HomeCard::Dapp { icon_url: (None) },
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(HomeCards::sample(), HomeCards::sample());
        assert_eq!(HomeCards::sample_other(), HomeCards::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(HomeCards::sample(), HomeCards::sample_other());
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(HomeCards::default().len(), 0);
    }
}
