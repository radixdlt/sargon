use crate::prelude::*;
use sargon::AuthorizedDappDetailed as InternalAuthorizedDappDetailed;

#[derive(
    Clone, Debug, PartialEq, Hash, Eq uniffi::Record,
)]
pub struct AuthorizedDappDetailed {
    pub network_id: NetworkID,

    pub dapp_definition_address: AccountAddress,

    pub display_name: Option<DisplayName>,

    pub detailed_authorized_personas: DetailedAuthorizedPersonas,

    pub preferences: AuthorizedDappPreferences,
}

impl From<InternalAuthorizedDappDetailed> for AuthorizedDappDetailed {
    fn from(value: InternalAuthorizedDappDetailed) -> Self {
        Self {
            network_id: value.network_id.into(),
            dapp_definition_address: value.dapp_definition_address.into(),
            display_name: value.display_name.map(Into::into),
            detailed_authorized_personas: value.detailed_authorized_personas.into(),
            preferences: value.preferences.into(),
        }
    }
}

impl Into<InternalAuthorizedDappDetailed> for AuthorizedDappDetailed {
    fn into(self) -> InternalAuthorizedDappDetailed {
        InternalAuthorizedDappDetailed {
            network_id: self.network_id.into(),
            dapp_definition_address: self.dapp_definition_address.into(),
            display_name: self.display_name.map(Into::into),
            detailed_authorized_personas: self.detailed_authorized_personas.into(),
            preferences: self.preferences.into(),
        }
    }
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample() -> AuthorizedDappDetailed {
    InternalAuthorizedDappDetailed::sample().into()
}

#[uniffi::export]
pub fn new_authorized_dapp_detailed_sample_other() -> AuthorizedDappDetailed {
    InternalAuthorizedDappDetailed::sample_other().into()
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
