use crate::prelude::*;

#[derive(
    Clone, Debug, PartialEq, Hash, Eq, derive_more::Display, uniffi::Record,
)]
#[display("{dapp_definition_address}")]
pub struct AuthorizedDappDetailed {
    pub network_id: NetworkID,

    pub dapp_definition_address: AccountAddress,

    pub display_name: Option<DisplayName>,

    pub detailed_authorized_personas: DetailedAuthorizedPersonas,

    pub preferences: AuthorizedDappPreferences,
}

impl Identifiable for AuthorizedDappDetailed {
    type ID = AccountAddress;

    fn id(&self) -> Self::ID {
        self.dapp_definition_address
    }
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample() -> AuthorizedDappDetailed {
    AuthorizedDappDetailed::sample()
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample_other() -> AuthorizedDappDetailed {
    AuthorizedDappDetailed::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthorizedDappDetailed;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_authorized_dapp_detailed_sample(),
                new_authorized_dapp_detailed_sample_other(),
                // duplicates should get removed
                new_authorized_dapp_detailed_sample(),
                new_authorized_dapp_detailed_sample_other(),
            ])
            .len(),
            2
        );
    }
}
