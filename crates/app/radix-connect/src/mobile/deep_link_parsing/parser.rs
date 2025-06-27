use super::request::RadixConnectMobileDappRequest;
use crate::prelude::*;

use core_misc::parse_url;

const CONNECT_URL_PARAM_SESSION_ID: &str = "sessionId";
const CONNECT_URL_PARAM_ORIGIN: &str = "origin";
const CONNECT_URL_PARAM_SIGNATURE: &str = "signature";
const CONNECT_URL_PARAM_INTERACTION: &str = "request";
const CONNECT_URL_PARAM_PUBLIC_KEY: &str = "publicKey";
const CONNECT_URL_PARAM_IDENTITY_KEY: &str = "identity";
const CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS: &str = "dAppDefinitionAddress";
const APP_SCHEME: &str = "radixwallet";

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::engine::Engine as _;

pub fn parse_mobile_connect_request(
    url: impl AsRef<str>,
) -> Result<RadixConnectMobileDappRequest> {
    let url = url.as_ref();

    let parsed_url = parse_url(url).map_err(|_| {
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        }
    })?;

    if parsed_url.scheme() != APP_SCHEME {
        return Err(CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        });
    }

    let query_parameters = parsed_url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();

    let session_id_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_SESSION_ID)?;
    let origin_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_ORIGIN)?;
    let public_key_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_PUBLIC_KEY)?;
    let request_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_INTERACTION)?;

    // The dApp sends WalletInteraction as base64_url encoded in the deepLink url.
    let decoded_request = URL_SAFE_NO_PAD
        .decode(request_string.as_str())
        .map_err(|_| CommonError::RadixConnectMobileInvalidRequestFormat)?;
    let request = decoded_request.deserialize()?;

    let dapp_definition_address_string = get_key(
        url,
        &query_parameters,
        CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS,
    )?;
    let signature_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_SIGNATURE)?;
    let identity_public_key_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_IDENTITY_KEY)?;

    let origin = DappOrigin::new(origin_string);
    let session_id =
        SessionID::from_str(session_id_string.as_ref()).map_err(|_| {
            CommonError::RadixConnectMobileInvalidSessionID {
                bad_value: session_id_string,
            }
        })?;

    let public_key = KeyAgreementPublicKey::from_hex(public_key_string)?;
    let identity_public_key =
        Ed25519PublicKey::from_hex(identity_public_key_string)?;
    let dapp_definition_address =
        DappDefinitionAddress::from_str(&dapp_definition_address_string)?;
    let signature = Ed25519Signature::from_hex(signature_string)?;

    let request = RadixConnectMobileDappRequest::new(
        session_id,
        origin,
        public_key,
        identity_public_key,
        dapp_definition_address,
        signature,
        request,
    );
    request.verify_request_signature()?;
    Ok(request)
}

fn get_key(
    url: &str,
    query_parameters: &HashMap<String, String>,
    key: &str,
) -> Result<String> {
    query_parameters
        .get(key)
        .ok_or(CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        })
        .map(|value| value.to_owned())
}

#[cfg(test)]
#[derive(Deserialize, Clone)]
pub struct SampleRequestParams {
    pub session_id: Option<String>,
    pub origin: Option<String>,
    pub public_key: Option<String>,
    pub identity_public_key: Option<String>,
    pub request: Option<String>,
    pub dapp_definition_address: Option<String>,
    pub signature: Option<String>,
}

#[cfg(test)]
impl SampleRequestParams {
    pub fn build_base_url_with_scheme(&self, scheme: &str) -> String {
        let mut params: Vec<String> = Vec::new();
        if let Some(session_id) = &self.session_id {
            let str =
                format!("{}={}", CONNECT_URL_PARAM_SESSION_ID, session_id);
            params.push(str);
        }

        if let Some(origin) = &self.origin {
            let str = format!("{}={}", CONNECT_URL_PARAM_ORIGIN, origin);
            params.push(str);
        }

        if let Some(public_key) = &self.public_key {
            let str =
                format!("{}={}", CONNECT_URL_PARAM_PUBLIC_KEY, public_key);
            params.push(str);
        }

        if let Some(request) = &self.request {
            let str = format!("{}={}", CONNECT_URL_PARAM_INTERACTION, request);
            params.push(str);
        }

        if let Some(dapp_definition_address) = &self.dapp_definition_address {
            let str = format!(
                "{}={}",
                CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS,
                dapp_definition_address
            );
            params.push(str);
        }

        if let Some(signature) = &self.signature {
            let str = format!("{}={}", CONNECT_URL_PARAM_SIGNATURE, signature);
            params.push(str);
        }

        if let Some(identity_public_key) = &self.identity_public_key {
            let str = format!(
                "{}={}",
                CONNECT_URL_PARAM_IDENTITY_KEY, identity_public_key
            );
            params.push(str);
        }

        format!("{}://?{}", scheme, params.join("&"))
    }

    pub fn build_base_url(&self) -> String {
        self.build_base_url_with_scheme(APP_SCHEME)
    }

    pub fn new_from_text_vector() -> Self {
        fixture::<SampleRequestParams>(prelude::fixture_interaction!(
            "deep_link_request_params"
        ))
        .unwrap()
    }

    /// The default interaction that is encoded in the test vector deep_link_request_params.json
    pub fn test_vector_encoded_interaction(
    ) -> DappToWalletInteractionUnvalidated {
        DappToWalletInteractionUnvalidated::new(
            "011cee03-961a-4e55-b69b-6ecad0b068c7".parse().unwrap(),
            DappToWalletInteractionItems::AuthorizedRequest(
                DappToWalletInteractionAuthorizedRequestItems::new(
                    DappToWalletInteractionAuthRequestItem::LoginWithChallenge(
                        DappToWalletInteractionAuthLoginWithChallengeRequestItem::new(
                            DappToWalletInteractionAuthChallengeNonce(
                                Exactly32Bytes::from_hex("17ca9f21bd1b38db43923025d7de1311d6f598989474f1434ded8eed806d2c57").unwrap()
                            )
                        )
                    ),
                    Some(DappToWalletInteractionResetRequestItem::new(false, false)),
                    None,
                    None,
                    None,
                    None,
                    None,
                )
            ),
            DappToWalletInteractionMetadataUnvalidated::new(
                WalletInteractionVersion(2),
                NetworkID::Stokenet,
                DappOrigin::from("https://d1vq8n3dnxcyhd.cloudfront.net"),
                "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe",
            )
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_connect_url_params() {
        pretty_assertions::assert_eq!(
            CONNECT_URL_PARAM_SESSION_ID,
            "sessionId"
        );
        pretty_assertions::assert_eq!(CONNECT_URL_PARAM_ORIGIN, "origin");
        pretty_assertions::assert_eq!(CONNECT_URL_PARAM_SIGNATURE, "signature");
        pretty_assertions::assert_eq!(CONNECT_URL_PARAM_INTERACTION, "request");
        pretty_assertions::assert_eq!(
            CONNECT_URL_PARAM_PUBLIC_KEY,
            "publicKey"
        );
        pretty_assertions::assert_eq!(
            CONNECT_URL_PARAM_IDENTITY_KEY,
            "identity"
        );
        pretty_assertions::assert_eq!(
            CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS,
            "dAppDefinitionAddress"
        );
        pretty_assertions::assert_eq!(APP_SCHEME, "radixwallet");
    }

    use base64::engine::general_purpose::URL_SAFE_NO_PAD;

    #[test]
    fn parse_url_into_request() {
        let request_params = SampleRequestParams::new_from_text_vector();

        let connect_url = request_params.build_base_url();
        let result = parse_mobile_connect_request(connect_url);

        let expected_interaction =
            SampleRequestParams::test_vector_encoded_interaction();

        let expected_request = RadixConnectMobileDappRequest::new(
            request_params.session_id.clone().unwrap().parse().unwrap(),
            DappOrigin::from(request_params.origin.clone().unwrap().as_str()),
            KeyAgreementPublicKey::from_hex(
                request_params.public_key.clone().unwrap(),
            )
            .unwrap(),
            Ed25519PublicKey::from_hex(
                request_params.identity_public_key.clone().unwrap(),
            )
            .unwrap(),
            request_params
                .dapp_definition_address
                .clone()
                .unwrap()
                .parse()
                .unwrap(),
            request_params.signature.clone().unwrap().parse().unwrap(),
            expected_interaction,
        );

        pretty_assertions::assert_eq!(result, Ok(expected_request));

        let parsed_request = result.unwrap();
        let expected_interaction = URL_SAFE_NO_PAD
            .decode(request_params.request.unwrap())
            .unwrap()
            .deserialize()
            .unwrap();
        pretty_assertions::assert_eq!(
            parsed_request.interaction,
            expected_interaction
        );
        pretty_assertions::assert_eq!(
            parsed_request.session_id.to_string(),
            request_params.session_id.unwrap()
        );
        pretty_assertions::assert_eq!(
            parsed_request.origin.to_string(),
            request_params.origin.unwrap()
        );
        pretty_assertions::assert_eq!(
            parsed_request.public_key.to_hex(),
            request_params.public_key.unwrap()
        );
        pretty_assertions::assert_eq!(
            parsed_request.identity_public_key.to_hex(),
            request_params.identity_public_key.unwrap()
        );
        pretty_assertions::assert_eq!(
            parsed_request.dapp_definition_address.to_string(),
            request_params.dapp_definition_address.unwrap()
        );
        pretty_assertions::assert_eq!(
            parsed_request.signature.to_hex(),
            request_params.signature.unwrap()
        );
    }

    #[test]
    fn parse_url_with_invalid_scheme() {
        let request_params = SampleRequestParams::new_from_text_vector();
        let invalid_scheme_url =
            request_params.build_base_url_with_scheme("invalidScheme");
        let result = parse_mobile_connect_request(invalid_scheme_url.clone());

        pretty_assertions::assert_ne!(
            parse_url(invalid_scheme_url.clone()).unwrap().scheme(),
            APP_SCHEME
        );
        assert!(matches!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl { .. })
        ));
    }

    #[test]
    fn parse_url_missing_query_param() {
        let missing_param_url = format!("{}://", APP_SCHEME);
        let result = parse_mobile_connect_request(missing_param_url);
        assert!(matches!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl { .. })
        ));
    }

    #[test]
    fn parse_invalid_url() {
        let invalid_url = "invalid_url";
        let result = parse_mobile_connect_request(invalid_url);
        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: invalid_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_with_invalid_request() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.request = "invalid_request".to_string().into();
        let invalid_request_url = request.build_base_url();
        let result = parse_mobile_connect_request(invalid_request_url);
        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestFormat)
        );
    }

    #[test]
    fn parse_url_with_invalid_session_id() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.session_id = "invalid_session_id".to_string().into();

        let invalid_session_id_url = request.build_base_url();
        let result = parse_mobile_connect_request(invalid_session_id_url);
        assert!(matches!(
            result,
            Err(CommonError::RadixConnectMobileInvalidSessionID { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_public_key() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.public_key = "invalid_public_key".to_string().into();

        let invalid_public_key_url = request.build_base_url();
        let result = parse_mobile_connect_request(invalid_public_key_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidKeyAgreementPublicKeyFromHex { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_signature() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.signature = "invalid_signature".to_string().into();

        let invalid_signature_url = request.build_base_url();
        let result = parse_mobile_connect_request(invalid_signature_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidEd25519SignatureFromString { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_identity_key() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.identity_public_key = "invalid_identity_key".to_string().into();

        let invalid_identity_key_url = request.build_base_url();
        let result = parse_mobile_connect_request(invalid_identity_key_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidEd25519PublicKeyFromString { .. })
        ));
    }

    #[test]
    fn parse_url_missing_session_id() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.session_id = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_missing_origin() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.origin = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_missing_interaction() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.request = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_missing_dapp_definition_address() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.dapp_definition_address = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_missing_signature() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.signature = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_missing_identity() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.identity_public_key = None;

        assert_deep_link_url_is_invalid(request);
    }

    #[test]
    fn parse_url_with_invalid_dapp_definition_address() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.dapp_definition_address = "invalid".to_string().into();

        let invalid_dapp_definition_address_url = request.build_base_url();
        let result =
            parse_mobile_connect_request(invalid_dapp_definition_address_url);
        assert_eq!(
            result,
            Err(CommonError::FailedToFindNetworkIdFromBech32mString {
                bech32m_encoded_address: "invalid".to_string()
            })
        );
    }

    #[test]
    fn parse_url_signature_validation_failed() {
        let mut request = SampleRequestParams::new_from_text_vector();
        request.signature = "93bc8fd33cdbd56bc1f7a9b46afc9615b5b42e9aad63227e71b02c57eb88f5f166406182afa82ebe8eb3bfc9e1388adfd60670d098751b1507584999be36c50f".to_string().into();

        let wrogn_signature_url = request.build_base_url();
        let result = parse_mobile_connect_request(wrogn_signature_url);
        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidDappSignature)
        );
    }

    fn assert_deep_link_url_is_invalid(request: SampleRequestParams) {
        let bad_url = request.build_base_url();
        let result = parse_mobile_connect_request(&bad_url);
        pretty_assertions::assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: bad_url.to_owned(),
            })
        );
    }
}
