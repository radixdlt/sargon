use crate::prelude::*;

const ROLA_PREFIX: u8 = 0x52;

#[derive(
    Debug,
    Clone,
    PartialEq,
    derive_more::Display,
)]
#[display("{}", self.payload.to_hex())]
pub struct RolaChallenge {
    payload: BagOfBytes,
}

impl RolaChallenge {
    pub fn hash(&self) -> Hash {
        hash_of(self.payload.clone())
    }
}

impl HasSampleValues for RolaChallenge {
    fn sample() -> Self {
        Self::from_request(
            DappToWalletInteractionAuthChallengeNonce::sample(),
            DappToWalletInteractionMetadata::sample()
        ).unwrap()
    }

    fn sample_other() -> Self {
        Self::from_request(
            DappToWalletInteractionAuthChallengeNonce::sample_other(),
            DappToWalletInteractionMetadata::sample_other()
        ).unwrap()
    }
}

impl RolaChallenge {

    /// Constructs a payload to sign in conjunction with the `challenge` received and
    /// the `metadata` of the dApp that sent the request.
    ///
    /// The logic of constructing the payload is as follows:
    /// * Prefixes with constant `ROLA_PREFIX` (0x52)
    /// * Extends with the 32 raw bytes of the challenge
    /// * Pushes 1 byte which is the length of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the bech32-encoded dapp-definition address
    /// * Extends with the bytes of the origin UTF-8 encoded.
    ///
    /// Fails if the `origin` Url is not a valid url
    pub fn from_request(
        challenge: DappToWalletInteractionAuthChallengeNonce,
        metadata: DappToWalletInteractionMetadata,
    ) -> Result<Self> {
        TryInto::<Url>::try_into(metadata.origin.clone())?;
        let mut origin_str = metadata.origin.0;
        if origin_str.ends_with("/") {
            origin_str.truncate(origin_str.len() - 1);
        }

        let mut payload = Vec::<u8>::new();
        payload.push(ROLA_PREFIX);
        payload.extend(challenge.0.bytes());
        payload.push(metadata.dapp_definition_address.address().len() as u8);
        payload.extend(metadata.dapp_definition_address.address().bytes());
        payload.extend(origin_str.as_bytes());

        Ok(RolaChallenge {
            payload: BagOfBytes::from(payload)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(
        Debug,
        Clone,
        Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    struct RolaChallengeVectorItem {
        pub payload_to_hash: BagOfBytes,
        #[serde(deserialize_with = "deserialize_hash")]
        pub blake_hash_of_payload: Hash,
        pub d_app_definition_address: AccountAddress,
        pub origin: DappOrigin,
        pub challenge: Exactly32Bytes
    }

    fn deserialize_hash<'de, D>(deserializer: D) -> Result<Hash, D::Error>
    where D: Deserializer<'de> {
        let buf = String::deserialize(deserializer)?;

        Hash::from_str(&buf).map_err(de::Error::custom)
    }

    impl TryInto<RolaChallenge> for RolaChallengeVectorItem {
        type Error = CommonError;

        fn try_into(self) -> std::result::Result<RolaChallenge, Self::Error> {
            RolaChallenge::from_request(
                DappToWalletInteractionAuthChallengeNonce(self.challenge),
                DappToWalletInteractionMetadata::new(
                    WalletInteractionVersion::current(),
                    self.d_app_definition_address.network_id(),
                    self.origin,
                    self.d_app_definition_address
                )
            )
        }
    }

    #[test]
    fn test_from_vectors() {
        let json = include_str!(concat!(
            env!("FIXTURES_VECTOR"),
            "rola_challenge_payload_hash_vectors.json"
        ));
        let vector = serde_json::from_str::<Vec<RolaChallengeVectorItem>>(json).unwrap();

        vector.iter().for_each(|v| {
            let rola_challenge: RolaChallenge = v.clone().try_into().unwrap();

            assert_eq!(
                rola_challenge.payload,
                v.payload_to_hash
            );

            assert_eq!(
                rola_challenge.hash(),
                v.blake_hash_of_payload
            );
        });
    }

    #[test]
    fn test_valid_url() {
        let nonce = Exactly32Bytes::sample();
        let challenge = RolaChallenge::from_request(
            DappToWalletInteractionAuthChallengeNonce(nonce),
            metadata("https://stokenet-dashboard.radixdlt.com"),
        ).unwrap();

        let expected_payload_hex = "52deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead\
        dead426163636f756e745f72647831323879366a37386d74306171763633373265767a323868727870386d6e30\
        36636364646b7237787070633838687976796e766a64777268747470733a2f2f73746f6b656e65742d64617368\
        626f6172642e7261646978646c742e636f6d";

        assert_eq!(
            expected_payload_hex,
            challenge.payload.to_hex()
        );

        let challenge_with_origin_with_slash = RolaChallenge::from_request(
            DappToWalletInteractionAuthChallengeNonce(nonce),
            metadata("https://stokenet-dashboard.radixdlt.com/"),
        ).unwrap();

        assert_eq!(
            expected_payload_hex,
            challenge_with_origin_with_slash.payload.to_hex()
        );
    }

    #[test]
    fn test_invalid_url() {
        let nonce = Exactly32Bytes::sample();
        let challenge = RolaChallenge::from_request(
            DappToWalletInteractionAuthChallengeNonce(nonce),
            metadata("/"),
        );

        assert_eq!(
            challenge,
            Err(CommonError::InvalidURL { bad_value: "/".to_string() })
        );
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