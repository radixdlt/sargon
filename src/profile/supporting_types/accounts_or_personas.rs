use crate::prelude::*;

decl_identified_vec_of!(
    /// An ordered set of entities of mixed type, either [`Account`] or [`Persona`].
    AccountsOrPersonas,
    AccountOrPersona
);

impl HasSampleValues for AccountsOrPersonas {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::sample_mainnet()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::sample_stokenet()
    }
}

impl AccountsOrPersonas {
    pub(crate) fn sample_mainnet() -> Self {
        Self::from_iter([
            Account::sample_mainnet().into(),
            Persona::sample_mainnet().into(),
            Persona::sample_mainnet_other().into(),
            Account::sample_mainnet_other().into(),
            Account::sample_mainnet_third().into(),
            Persona::sample_mainnet_third().into(),
        ])
    }

    pub(crate) fn sample_stokenet() -> Self {
        Self::from_iter([
            Persona::sample_stokenet().into(),
            Account::sample_stokenet().into(),
            Account::sample_stokenet_other().into(),
            Persona::sample_stokenet_other().into(),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountsOrPersonas;

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
