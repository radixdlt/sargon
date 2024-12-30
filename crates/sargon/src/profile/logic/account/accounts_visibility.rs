use crate::prelude::*;

pub trait VisibleOrHidden {
    fn visible(&self) -> Self;
    fn hidden(&self) -> Self;
}

impl VisibleOrHidden for Accounts {
    fn visible(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| !p.is_hidden() && !p.is_tombstoned())
            .collect()
    }

    fn hidden(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| p.is_hidden() && !p.is_tombstoned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Accounts;

    #[test]
    fn test_get_non_hidden_none_hidden() {
        let sut = SUT::sample();
        assert_eq!(&sut.visible(), &sut)
    }

    #[test]
    fn test_get_visible_one_visible() {
        let values = &[
            Account::sample_mainnet_bob(),
            Account::sample_mainnet_diana(), // This account is hidden
            Account::sample_mainnet_erin(),  // This account is tombstoned
        ];
        let sut = SUT::from_iter(values.clone());

        assert_eq!(sut.visible(), SUT::just(Account::sample_mainnet_bob()))
    }

    #[test]
    fn hidden() {
        let values = &[
            Account::sample_mainnet_bob(),
            Account::sample_mainnet_diana(), // This account is hidden
            Account::sample_mainnet_erin(),  // This account is tombstoned
        ];
        let sut = SUT::from_iter(values.clone());

        assert_eq!(sut.hidden(), SUT::just(Account::sample_mainnet_diana()))
    }
}
