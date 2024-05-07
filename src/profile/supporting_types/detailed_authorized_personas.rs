use crate::prelude::*;

decl_ordered_map!(DetailedAuthorizedPersonas, AuthorizedPersonaDetailed);

impl HasSampleValues for DetailedAuthorizedPersonas {
    fn sample() -> Self {
        Self::from_iter([
            AuthorizedPersonaDetailed::sample(),
            AuthorizedPersonaDetailed::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([AuthorizedPersonaDetailed::sample_other()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DetailedAuthorizedPersonas;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
