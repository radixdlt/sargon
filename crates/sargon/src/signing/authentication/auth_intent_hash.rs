use crate::prelude::*;

const ROLA_PREFIX: u8 = 0x52;

#[derive(
    Debug, Clone, PartialEq, Eq, derive_more::Display, std::hash::Hash,
)]
#[display("{}", self.payload.to_hex())]
pub struct AuthIntentHash {
    pub payload: BagOfBytes,
}

impl AuthIntentHash {
    pub fn hash(&self) -> Hash {
        hash_of(self.payload.clone())
    }
}

impl From<AuthIntentHash> for Hash {
    fn from(val: AuthIntentHash) -> Self {
        val.hash()
    }
}

impl HasSampleValues for AuthIntentHash {
    fn sample() -> Self {
        From::<AuthIntent>::from(AuthIntent::sample())
    }

    fn sample_other() -> Self {
        From::<AuthIntent>::from(AuthIntent::sample_other())
    }
}

impl From<AuthIntent> for AuthIntentHash {
    /// Constructs a payload to sign in conjunction with the `challenge_nonce` received and
    /// the `metadata` of the dApp that sent the request.
    ///
    /// The logic of constructing the payload is as follows:
    /// * Prefixes with constant `ROLA_PREFIX` (0x52)
    /// * Extends with the 32 raw bytes of the challenge
    /// * Pushes 1 byte which is the length of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the origin UTF-8 encoded.
    fn from(value: AuthIntent) -> Self {
        let mut origin_str = value.origin.to_string();
        if origin_str.ends_with("/") {
            origin_str.truncate(origin_str.len() - 1);
        }

        let mut payload = Vec::<u8>::new();
        payload.push(ROLA_PREFIX);
        payload.extend(value.challenge_nonce.bytes());
        payload.push(value.dapp_definition_address.address().len() as u8);
        payload.extend(value.dapp_definition_address.address().bytes());
        payload.extend(origin_str.as_bytes());

        Self {
            payload: BagOfBytes::from(payload),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let json = include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "rola_challenge_payload_hash_vectors.json"
        ));
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

        let challenge_with_origin_with_slash =
            sut(nonce, metadata("https://stokenet-dashboard.radixdlt.com/"))
                .unwrap();

        assert_eq!(
            expected_payload_hex,
            challenge_with_origin_with_slash.payload.to_hex()
        );
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
