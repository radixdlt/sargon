#[cfg(test)]
mod tests {
    use prelude::fixture_interaction;
    use serde::{de, Deserializer};

    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AuthIntentHash;

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AuthIntentHashVectorItem {
        pub payload_to_hash: BagOfBytes,
        #[serde(deserialize_with = "deserialize_hash")]
        pub blake_hash_of_payload: Hash,
        pub d_app_definition_address: AccountAddress,
        pub origin: DappOrigin,
        pub challenge: Exactly32Bytes,
    }

    fn deserialize_hash<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        let buf = String::deserialize(deserializer)?;

        Hash::from_str(&buf).map_err(de::Error::custom)
    }

    impl TryInto<SUT> for AuthIntentHashVectorItem {
        type Error = CommonError;

        fn try_into(self) -> std::result::Result<SUT, Self::Error> {
            let network_id = self.d_app_definition_address.network_id();
            let intent = AuthIntent::new_from_request(
                DappToWalletInteractionAuthChallengeNonce(self.challenge),
                DappToWalletInteractionMetadata::new(
                    WalletInteractionVersion::current(),
                    network_id,
                    self.origin,
                    self.d_app_definition_address,
                ),
                vec![AddressOfAccountOrPersona::Account(
                    AccountAddress::random(network_id),
                )],
            )?;

            Ok(From::<AuthIntent>::from(intent))
        }
    }

    #[test]
    fn test_from_vectors() {
        let json = fixture_interaction!("rola_challenge_payload_hash_vectors");
        let vector =
            serde_json::from_str::<Vec<AuthIntentHashVectorItem>>(json)
                .unwrap();

        vector.iter().for_each(|v| {
            let rola_challenge: SUT = v.clone().try_into().unwrap();

            assert_eq!(rola_challenge.payload, v.payload_to_hash);

            assert_eq!(rola_challenge.hash(), v.blake_hash_of_payload);
        });
    }

    #[test]
    fn test_valid_url() {
        let nonce = Exactly32Bytes::sample();
        let challenge =
            sut(nonce, metadata("https://stokenet-dashboard.radixdlt.com"))
                .unwrap();

        let expected_payload_hex = "52deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead\
        dead426163636f756e745f72647831323879366a37386d74306171763633373265767a323868727870386d6e30\
        36636364646b7237787070633838687976796e766a64777268747470733a2f2f73746f6b656e65742d64617368\
        626f6172642e7261646978646c742e636f6d";

        assert_eq!(expected_payload_hex, challenge.payload.to_hex());
    }

    #[test]
    fn test_invalid_url() {
        let nonce = Exactly32Bytes::sample();
        let challenge = sut(nonce, metadata("/"));

        assert_eq!(
            challenge,
            Err(CommonError::InvalidURL {
                bad_value: "/".to_string()
            })
        );
    }

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    fn sut(
        nonce: Exactly32Bytes,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<SUT> {
        let intent = AuthIntent::new_from_request(
            DappToWalletInteractionAuthChallengeNonce(nonce),
            metadata.clone(),
            [AddressOfAccountOrPersona::Account(AccountAddress::random(
                metadata.network_id,
            ))],
        )?;

        Ok(From::<AuthIntent>::from(intent))
    }

    fn metadata(url: impl Into<String>) -> DappToWalletInteractionMetadata {
        DappToWalletInteractionMetadata::new(
            WalletInteractionVersion(1),
            NetworkID::Mainnet,
            DappOrigin(url.into()),
            AccountAddress::sample(),
        )
    }
}
