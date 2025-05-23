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

pub trait Sortable {
    fn sort(&self) -> Self;
}

impl Sortable for HomeCards {
    fn sort(&self) -> Self {
        let mut vec = self.into_iter().collect_vec();
        vec.sort();
        Self::from_iter(vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = HomeCards;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(SUT::default().len(), 0);
    }

    #[test]
    fn sort() {
        let result =
            SUT::from_iter(vec![HomeCard::Connector, HomeCard::StartRadQuest])
                .sort()
                .items();
        let expected_result =
            SUT::from_iter(vec![HomeCard::StartRadQuest, HomeCard::Connector])
                .items();
        pretty_assertions::assert_eq!(result, expected_result);
    }
}
