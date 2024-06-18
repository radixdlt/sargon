use crypto::signatures::ed25519::Signature;

use super::super::session::session_id::SessionID;
use crate::prelude::*;
use hex::ToHex;

#[derive(Debug, PartialEq)]
pub struct RadixConnectMobileRequest {
    pub session_id: SessionID,
    pub origin: DappOrigin,
    pub public_key: KeyAgreementPublicKey,
    pub identity_public_key: Ed25519PublicKey,
    pub dapp_definition_address: DappDefinitionAddress,
    pub signature: Ed25519Signature,
    pub request: DappToWalletInteractionUnvalidated,
}

impl RadixConnectMobileRequest {
    pub fn new(
        session_id: SessionID,
        origin: DappOrigin,
        public_key: KeyAgreementPublicKey,
        identity_public_key: Ed25519PublicKey,
        dapp_definition_address: DappDefinitionAddress,
        signature: Ed25519Signature,
        request: DappToWalletInteractionUnvalidated,
    ) -> Self {
        Self {
            session_id,
            origin,
            public_key,
            identity_public_key,
            dapp_definition_address,
            signature,
            request,
        }
    }
}

impl HasSampleValues for RadixConnectMobileRequest {
    fn sample() -> Self {
        RadixConnectMobileRequest::new(
            SessionID::sample(),
            DappOrigin::sample(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample(),
            DappDefinitionAddress::sample(),
            Ed25519Signature::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }

    fn sample_other() -> Self {
        RadixConnectMobileRequest::new(
            SessionID::sample(),
            DappOrigin::sample_other(),
            KeyAgreementPublicKey::sample(),
            Ed25519PublicKey::sample_other(),
            DappDefinitionAddress::sample(),
            Ed25519Signature::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }
}

impl RadixConnectMobileRequest {
    pub fn verify_signature(
        &self,
        interaction_id: &WalletInteractionId,
    ) -> Result<bool> {
        let length_of_dapp_def_address =
            self.dapp_definition_address.address().len(); // Replace this with the actual length value
        let length_of_dapp_def_address_hex =
            format!("{:x}", length_of_dapp_def_address);

        let message: String = [
            "C".as_bytes().encode_hex(),
            interaction_id.0.as_bytes().encode_hex(),
            length_of_dapp_def_address_hex,
            self.dapp_definition_address
                .address()
                .as_bytes()
                .encode_hex(),
            self.origin.0.as_bytes().encode_hex(),
        ]
        .concat();

        let hash: Hash = hash_of(hex_decode(message).unwrap());
        Ok(self
            .identity_public_key
            .is_valid_signature_for_hash(&self.signature, &hash))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn signature_verification() {
        // let interaction_id = WalletInteractionId::from_str(
        //     "a006e3df-7f28-43c5-a1c7-eb34641bdcc5",
        // )
        // .unwrap();
        // let request = RadixConnectMobileRequest::new(
        //     SessionID::sample(),
        //     DappOrigin::new("https://d3kgzcz7d65kcn.cloudfront.net"),
        //     KeyAgreementPublicKey::from_hex("a3bb59f33eed65fce017558f25b6ef9f073bbb4412b893d1d6babebc45c8e55b".to_string()).unwrap(),
        //     Ed25519PublicKey::from_hex("a3bb59f33eed65fce017558f25b6ef9f073bbb4412b893d1d6babebc45c8e55b".to_string()).unwrap(),
        //     DappDefinitionAddress::from_str("account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe").unwrap(),
        //     Ed25519Signature::from_hex("9ceedf5d1dbdcd5d4a36d859fddbd9fa913e36d27afa6cf9bd54206d3e86350b0f4322181ee4222a2dbd2e8bac90611d4eb6982914458b7bc59a51ff5cf9fd09".to_string()).unwrap(),
        //     DappToWalletInteractionUnvalidated::sample_with_interaction_id(interaction_id.clone()),
        // );

        // pretty_assertions::assert_eq!(
        //     request.verify_signature(interaction_id),
        //     Ok(true)
        // );
    }
}
