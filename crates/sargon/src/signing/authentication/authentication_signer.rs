use crate::prelude::*;

pub struct AuthenticationSigner {
    input: AuthenticationSigningInput,
    interactor: Arc<dyn AuthenticationSigningInteractor>,
}

impl AuthenticationSigner {
    pub fn new(
        interactor: Arc<dyn AuthenticationSigningInteractor>,
        profile: &Profile,
        address_of_entity: AddressOfAccountOrPersona,
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<Self> {
        let input = AuthenticationSigningInput::try_from_profile(
            profile,
            address_of_entity,
            challenge_nonce,
            metadata,
        )?;

        Ok(Self { input, interactor })
    }

    pub async fn sign(self) -> Result<WalletToDappInteractionAuthProof> {
        self.interactor
            .sign(self.input.clone().into())
            .await
            .map(WalletToDappInteractionAuthProof::from)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthenticationSigner;

    #[actix_rt::test]
    async fn test_with_account_in_profile() {
        let factor_sources = FactorSource::sample_all();

        let factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fia0();
        let account = Account::sample_unsecurified_mainnet(
            "Unsecurified",
            factor_instance.clone(),
        );
        let sut = SUT::new(
            Arc::new(TestAuthenticationInteractor::new_succeeding()),
            &Profile::sample_from(factor_sources, [&account], []),
            AddressOfAccountOrPersona::from(account.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        let result = sut.sign().await.unwrap();

        assert_eq!(
            result,
            WalletToDappInteractionAuthProof::new(
                factor_instance.clone().public_key.public_key,
                Signature::try_from(
                    BagOfBytes::from_hex(
                        "eaa9a0eb1c9061e1999afa309db7bc9eecd30ad008f09cbfb2c4cf202759e914f6dbe01f67a7b8b1f1149f02b0e2662982fff4c1a765ee0d4d77651f1b91100c"
                    ).unwrap()
                ).unwrap()
            )
        );
    }

    #[actix_rt::test]
    async fn test_with_persona_in_profile() {
        let factor_sources = FactorSource::sample_all();

        let factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fii0();
        let persona = Persona::sample_unsecurified_mainnet(
            "Alice",
            factor_instance.clone(),
        );
        let sut = SUT::new(
            Arc::new(TestAuthenticationInteractor::new_succeeding()),
            &Profile::sample_from(factor_sources, [], [&persona]),
            AddressOfAccountOrPersona::from(persona.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        let result = sut.sign().await.unwrap();

        assert_eq!(
            result,
            WalletToDappInteractionAuthProof::new(
                factor_instance.clone().public_key.public_key,
                Signature::try_from(
                    BagOfBytes::from_hex(
                        "fb4f502e6a8bdbe3e66d365fe619270bafb9237e79a3c68dcf34448293f00840a0e5d48d1060eea49bf219cd3727c15cd6e305871956bc8d8ed8bcdc7fe97909"
                    ).unwrap()
                ).unwrap()
            )
        );
    }

    #[actix_rt::test]
    async fn test_with_failing_interactor() {
        let factor_sources = FactorSource::sample_all();

        let factor_instance =
            HierarchicalDeterministicFactorInstance::sample_fii0();
        let persona = Persona::sample_unsecurified_mainnet(
            "Alice",
            factor_instance.clone(),
        );
        let sut = SUT::new(
            Arc::new(TestAuthenticationInteractor::new_failing()),
            &Profile::sample_from(factor_sources, [], [&persona]),
            AddressOfAccountOrPersona::from(persona.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        let result = sut.sign().await;

        assert!(result.is_err());
    }

    #[actix_rt::test]
    async fn securified_account() {
        let factor_sources = FactorSource::sample_all();

        let securified_account = Account::sample_at(2);
        let factor_instance = securified_account
            .security_state()
            .as_securified()
            .unwrap()
            .authentication_signing_factor_instance();
        let sut = SUT::new(
            Arc::new(TestAuthenticationInteractor::new_succeeding()),
            &Profile::sample_from(factor_sources, [&securified_account], []),
            AddressOfAccountOrPersona::from(securified_account.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();

        let result = sut.sign().await.unwrap();

        pretty_assertions::assert_eq!(
            result,
            WalletToDappInteractionAuthProof::new(
                factor_instance.clone().public_key.public_key,
                "be359ca8edf8dccb2155e841952e88591fe42903da7e9a1560e863fe2fea94ed412d9eed282d201d48e5602fcde9a1dc201f440e86cb5f919b493bafc0ba2002".parse::<Signature>().unwrap()
            )
        );
    }

    #[actix_rt::test]
    async fn securified_account_with_failing_interactor() {
        let factor_sources = FactorSource::sample_all();

        let securified_account = Account::sample_at(2);
        let sut = SUT::new(
            Arc::new(TestAuthenticationInteractor::new_failing()),
            &Profile::sample_from(factor_sources, [&securified_account], []),
            AddressOfAccountOrPersona::from(securified_account.address),
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
        )
        .unwrap();
        let res = sut.sign().await;
        assert!(res.is_err());
    }
}
