use crate::prelude::*;
use std::hash::Hasher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthIntent {
    /// The challenge nonce that with some `metadata` values are generating the `RolaChallenge`
    /// needed to be signed
    pub challenge_nonce: Exactly32Bytes,

    /// The `NetworkID` on which the request was made
    pub network_id: NetworkID,

    /// The origin `Url` of the dApp from which the request was made
    pub origin: Url,

    /// The dApp's definition address
    pub dapp_definition_address: DappDefinitionAddress,

    /// The entities needed to be signed.
    pub entities_to_sign: IndexSet<AddressOfAccountOrPersona>,
}

impl AuthIntent {
    /// Creates an `AuthIntent` from the request received from a dApp.
    ///
    /// Fails if the `OriginUrl` in metadata is not a valid url or the network id supplied does not
    /// match the network id of the given dApp definition address or the entities to sign.
    pub fn new_from_request(
        challenge_nonce: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
        entities_to_sign: impl IntoIterator<Item = AddressOfAccountOrPersona>,
    ) -> Result<Self> {
        let origin = TryInto::<Url>::try_into(metadata.origin.clone())?;

        if metadata.network_id != metadata.dapp_definition_address.network_id()
        {
            return Err(CommonError::NetworkDiscrepancy {
                expected: metadata.network_id.to_string(),
                actual: metadata
                    .dapp_definition_address
                    .network_id()
                    .to_string(),
            });
        }

        let entities = entities_to_sign.into_iter().collect::<IndexSet<_>>();
        for entity in &entities {
            if entity.network_id() != metadata.network_id {
                return Err(CommonError::NetworkDiscrepancy {
                    expected: metadata.network_id.to_string(),
                    actual: entity.network_id().to_string(),
                });
            }
        }

        Ok(Self::new(
            challenge_nonce.0,
            metadata.network_id,
            origin,
            metadata.dapp_definition_address,
            entities,
        ))
    }

    pub fn new(
        challenge_nonce: Exactly32Bytes,
        network_id: NetworkID,
        origin: Url,
        dapp_definition_address: DappDefinitionAddress,
        entities_to_sign: IndexSet<AddressOfAccountOrPersona>,
    ) -> Self {
        Self {
            challenge_nonce,
            network_id,
            origin,
            dapp_definition_address,
            entities_to_sign,
        }
    }

    pub fn auth_intent_hash(&self) -> AuthIntentHash {
        From::<Self>::from(self.clone())
    }
}

impl std::hash::Hash for AuthIntent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.auth_intent_hash().payload.bytes())
    }
}

impl HasSampleValues for AuthIntent {
    fn sample() -> Self {
        Self::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample(),
            vec![AddressOfAccountOrPersona::sample()],
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new_from_request(
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
            DappToWalletInteractionMetadata::sample_other(),
            vec![AddressOfAccountOrPersona::sample_other()],
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthIntent;

    #[test]
    fn test_new_from_valid_request() {
        let challenge_nonce =
            DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::sample();

        assert!(SUT::new_from_request(
            challenge_nonce,
            metadata,
            vec![AddressOfAccountOrPersona::sample()]
        )
        .is_ok())
    }

    #[test]
    fn test_new_from_invalid_request_due_to_invalid_url() {
        let challenge_nonce =
            DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            DappOrigin("".to_string()),
            DappDefinitionAddress::sample_mainnet(),
        );

        assert_eq!(
            SUT::new_from_request(
                challenge_nonce,
                metadata,
                vec![AddressOfAccountOrPersona::sample()]
            ),
            Err(CommonError::InvalidURL {
                bad_value: "".to_string()
            })
        )
    }

    #[test]
    fn test_new_from_invalid_request_due_to_network_discrepancy_in_dapp_definition(
    ) {
        let challenge_nonce =
            DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::new(
            WalletInteractionVersion::current(),
            NetworkID::Mainnet,
            DappOrigin::sample(),
            DappDefinitionAddress::sample_stokenet(),
        );

        assert_eq!(
            SUT::new_from_request(
                challenge_nonce,
                metadata,
                vec![AddressOfAccountOrPersona::sample()]
            ),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Mainnet,
                actual: NetworkID::Stokenet,
            })
        )
    }

    #[test]
    fn test_new_from_invalid_request_due_to_network_discrepancy_in_entities() {
        let challenge_nonce =
            DappToWalletInteractionAuthChallengeNonce::sample();
        let metadata = DappToWalletInteractionMetadata::sample();

        assert_eq!(
            SUT::new_from_request(
                challenge_nonce,
                metadata,
                vec![AddressOfAccountOrPersona::sample_stokenet()]
            ),
            Err(CommonError::NetworkDiscrepancy {
                expected: NetworkID::Mainnet.to_string(),
                actual: NetworkID::Stokenet.to_string(),
            })
        )
    }
}
