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
    pub interaction: DappToWalletInteractionUnvalidated,
}

impl RadixConnectMobileRequest {
    pub fn new(
        session_id: SessionID,
        origin: DappOrigin,
        public_key: KeyAgreementPublicKey,
        identity_public_key: Ed25519PublicKey,
        dapp_definition_address: DappDefinitionAddress,
        signature: Ed25519Signature,
        interaction: DappToWalletInteractionUnvalidated,
    ) -> Self {
        Self {
            session_id,
            origin,
            public_key,
            identity_public_key,
            dapp_definition_address,
            signature,
            interaction,
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
            KeyAgreementPublicKey::sample_other(),
            Ed25519PublicKey::sample_other(),
            DappDefinitionAddress::sample(),
            Ed25519Signature::sample(),
            DappToWalletInteractionUnvalidated::sample(),
        )
    }
}

impl RadixConnectMobileRequest {
    pub fn verify_request_signature(&self) -> Result<()> {
        let message = self.message_for_signature();
        self.verify_message_signature(&message)
    }

    pub fn verify_message_signature(&self, message: &Hash) -> Result<()> {
        let is_valid_signature = self
            .identity_public_key
            .is_valid_signature_for_hash(&self.signature, message);

        if is_valid_signature {
            Ok(())
        } else {
            Err(CommonError::RadixConnectMobileInvalidDappSignature)
        }
    }

    fn message_for_signature(&self) -> Hash {
        dapp_message_for_signature(
            &self.interaction.interaction_id,
            &self.dapp_definition_address,
            &self.origin,
        )
    }
}

pub fn dapp_message_for_signature(
    interaction_id: &WalletInteractionId,
    dapp_definition_address: &DappDefinitionAddress,
    origin: &DappOrigin,
) -> Hash {
    let length_of_dapp_def_address = dapp_definition_address.address().len();
    let length_of_dapp_def_address_hex =
        format!("{:x}", length_of_dapp_def_address);

    let message: String = [
        "C".as_bytes().encode_hex(),
        interaction_id.0.as_bytes().encode_hex(),
        length_of_dapp_def_address_hex,
        dapp_definition_address.address().as_bytes().encode_hex(),
        origin.0.as_bytes().encode_hex(),
    ]
    .concat();

    hash_of(hex_decode(message).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileRequest;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn signature_verification() {
        let interaction_id = WalletInteractionId::from_str(
            "ca8f525f-446b-42ff-b119-642a445d3c71",
        )
        .unwrap();
        let request = RadixConnectMobileRequest::new(
            SessionID::sample(),
            DappOrigin::new("https://d2xmq49o1iddud.cloudfront.net"),
            KeyAgreementPublicKey::from_hex("a3bb59f33eed65fce017558f25b6ef9f073bbb4412b893d1d6babebc45c8e55b".to_string()).unwrap(),
            Ed25519PublicKey::from_hex("4f6e9ac218fbaefbb237e3421e43afa8def511aff5c7368dc11a14ce6d889e81".to_string()).unwrap(),
            DappDefinitionAddress::from_str("account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe").unwrap(),
            Ed25519Signature::from_hex("93bc8fd33cdbd56bc1f7a9b46afc9615b5b42e9aad63227e71b02c57eb88f5f166406182afa82ebe8eb3bfc9e1388adfd60670d098751b1507584999be36c50f".to_string()).unwrap(),
            DappToWalletInteractionUnvalidated::sample_with_interaction_id(interaction_id.clone()),
        );

        let expected_message = Hash::from(Exactly32Bytes::from_hex("29cdf41222be5236c5fefe341955083a25a7275e54a6ca1565d7571064792ace").unwrap());
        let message = request.message_for_signature();
        pretty_assertions::assert_eq!(message, expected_message);

        pretty_assertions::assert_eq!(
            request.verify_message_signature(&message),
            Ok(()),
        );

        pretty_assertions::assert_eq!(
            request.verify_request_signature(),
            Ok(()),
        )
    }
}
