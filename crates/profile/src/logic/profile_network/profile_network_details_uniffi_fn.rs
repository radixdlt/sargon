use crate::prelude::*;

#[uniffi::export]
pub fn profile_network_details_for_authorized_dapp(
    profile_network: &ProfileNetwork,
    dapp: &AuthorizedDapp,
) -> Result<AuthorizedDappDetailed> {
    profile_network.details_for_authorized_dapp(dapp)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetwork;

    #[test]
    fn test_profile_network_details_for_authorized_dapp() {
        let sut = SUT::sample();
        let dapp = AuthorizedDapp::sample();
        assert_eq!(
            profile_network_details_for_authorized_dapp(&sut, &dapp)
                .unwrap()
                .dapp_definition_address,
            dapp.dapp_definition_address
        )
    }
}
