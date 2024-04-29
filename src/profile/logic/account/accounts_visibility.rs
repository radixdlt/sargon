use crate::prelude::*;

impl Accounts {
    pub fn non_hidden(&self) -> Self {
        self.clone()
            .into_iter()
            .filter(|p| !p.is_hidden())
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
        assert_eq!(&sut.non_hidden(), &sut)
    }

    #[test]
    fn test_get_non_hidden_one_hidden() {
        let values = &[
            Account::sample_mainnet_bob(),
            Account::sample_mainnet_diana(),
        ];
        let sut = SUT::from_iter(values.clone());

        assert_eq!(sut.non_hidden(), SUT::just(Account::sample_mainnet_bob()))
    }
}
