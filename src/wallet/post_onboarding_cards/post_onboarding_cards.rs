use crate::prelude::*;

decl_identified_vec_of!(PostOnboardingCard);

impl HasSampleValues for PostOnboardingCards {
    fn sample() -> Self {
        Self::from_iter([
            PostOnboardingCard::sample_start_radquest(),
            PostOnboardingCard::sample_connector(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([
            PostOnboardingCard::sample_continue_radquest(),
            PostOnboardingCard::sample_dapp(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(
            PostOnboardingCards::sample(),
            PostOnboardingCards::sample()
        );
        assert_eq!(
            PostOnboardingCards::sample_other(),
            PostOnboardingCards::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(
            PostOnboardingCards::sample(),
            PostOnboardingCards::sample_other()
        );
    }

    #[test]
    fn default_is_empty() {
        assert_eq!(PostOnboardingCards::default().len(), 0);
    }
}
