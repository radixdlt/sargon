use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticationSigningInput {
    /// The account or identity address of the entity which signs the rola challenge,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,

    /// The challenge that will be signed by `owned_factor_instance`
    pub challenge: RolaChallenge,
}

impl AuthenticationSigningInput {
    pub fn new(
        owned_factor_instance: OwnedFactorInstance,
        challenge: RolaChallenge,
    ) -> Self {
        Self {
            owned_factor_instance,
            challenge,
        }
    }

    pub fn try_from_profile(
        profile: &Profile,
        address_of_entity: AddressOfAccountOrPersona,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<Self> {
        let security_state = match address_of_entity {
            AddressOfAccountOrPersona::Account(account_address) => profile
                .account_by_address(account_address)
                .map(|a| a.security_state),
            AddressOfAccountOrPersona::Identity(identity_address) => profile
                .persona_by_address(identity_address)
                .map(|a| a.security_state),
        }?;

        let factor_instance = match security_state {
            EntitySecurityState::Unsecured { value } => value
                .authentication_signing
                .unwrap_or(value.transaction_signing),
            EntitySecurityState::Securified { value: _ } => {
                panic!("Authentication signing not yet implemented for securified entities.")
            }
        };

        let owned_factor_instance =
            OwnedFactorInstance::new(address_of_entity, factor_instance);

        let challenge = RolaChallenge::from_request(challenge_nonce, metadata)?;

        Ok(Self::new(owned_factor_instance, challenge))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthenticationSigningInput;

    #[test]
    fn test_with_usecurified_account() {
        let expected_factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fia0();
        let unsecurified_account = Account::sample_unsecurified_mainnet(
            "Unsecurified",
            expected_factor_instance.clone(),
        );

        let factor_sources = FactorSource::sample_all();
        let profile =
            Profile::sample_from(factor_sources, [&unsecurified_account], []);

        let sut = SUT::try_from_profile(
            &profile,
            AddressOfAccountOrPersona::from(unsecurified_account.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        assert_eq!(
            sut.owned_factor_instance.owner,
            AddressOfAccountOrPersona::from(unsecurified_account.address)
        );
        assert_eq!(sut.owned_factor_instance.value, expected_factor_instance);

        assert_eq!(
            "6fc75ec1d5c00941dc587c0a07409da1740c423c337c323ba7bdf68d61d4dd8e"
                .parse::<Hash>()
                .unwrap(),
            sut.challenge.hash()
        )
    }

    #[test]
    fn test_with_usecurified_persona() {
        let expected_factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fii0();
        let unsecurified_persona = Persona::sample_unsecurified_mainnet(
            "Alice",
            expected_factor_instance.clone(),
        );

        let factor_sources = FactorSource::sample_all();
        let profile =
            Profile::sample_from(factor_sources, [], [&unsecurified_persona]);

        let sut = SUT::try_from_profile(
            &profile,
            AddressOfAccountOrPersona::from(unsecurified_persona.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        assert_eq!(
            sut.owned_factor_instance.owner,
            AddressOfAccountOrPersona::from(unsecurified_persona.address)
        );
        assert_eq!(sut.owned_factor_instance.value, expected_factor_instance);

        assert_eq!(
            "6fc75ec1d5c00941dc587c0a07409da1740c423c337c323ba7bdf68d61d4dd8e"
                .parse::<Hash>()
                .unwrap(),
            sut.challenge.hash()
        )
    }

    #[test]
    fn test_with_unknown_account_address() {
        let factor_sources = FactorSource::sample_all();
        let profile = Profile::sample_from(factor_sources, [], []);

        let sut = SUT::try_from_profile(
            &profile,
            AddressOfAccountOrPersona::sample_account_mainnet(),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        );

        assert_eq!(sut, Err(CommonError::UnknownAccount))
    }

    #[test]
    fn test_with_unknown_identity_address() {
        let factor_sources = FactorSource::sample_all();
        let profile = Profile::sample_from(factor_sources, [], []);

        let sut = SUT::try_from_profile(
            &profile,
            AddressOfAccountOrPersona::sample_identity_mainnet(),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        );

        assert_eq!(sut, Err(CommonError::UnknownPersona))
    }

    #[test]
    fn test_with_invalid_url() {
        let expected_factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fii0();
        let unsecurified_persona = Persona::sample_unsecurified_mainnet(
            "Alice",
            expected_factor_instance.clone(),
        );

        let factor_sources = FactorSource::sample_all();
        let profile =
            Profile::sample_from(factor_sources, [], [&unsecurified_persona]);

        let sut = SUT::try_from_profile(
            &profile,
            AddressOfAccountOrPersona::from(unsecurified_persona.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::new(
                WalletInteractionVersion::current(),
                NetworkID::Mainnet,
                DappOrigin("/".to_string()),
                AccountAddress::sample_mainnet(),
            ),
        );

        assert_eq!(
            sut,
            Err(CommonError::InvalidURL {
                bad_value: "/".to_string()
            })
        )
    }
}
