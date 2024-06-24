use super::request::RadixConnectMobileRequest;
use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use crypto::signatures::ed25519::Signature;
use url::form_urlencoded;
use url::Url;

use super::*;

const CONNECT_URL_PARAM_SESSION_ID: &str = "sessionId";
const CONNECT_URL_PARAM_ORIGIN: &str = "origin";
const CONNECT_URL_PARAM_SIGNATURE: &str = "signature";
const CONNECT_URL_PARAM_INTERACTION: &str = "request";
const CONNECT_URL_PARAM_PUBLIC_KEY: &str = "publicKey";
const CONNECT_URL_PARAM_IDENTITY_KEY: &str = "identity";
const CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS: &str = "dAppDefinitionAddress";
const APP_SCHEME: &str = "radixwallet";

pub fn parse_mobile_connect_request(
    url: impl AsRef<str>,
) -> Result<RadixConnectMobileRequest> {
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

    let decoded_request = URL_SAFE_NO_PAD
        .decode(request_string.as_str())
        .map_err(|_| CommonError::RadixConnectMobileInvalidRequestFormat)?;
    let request = DappToWalletInteractionUnvalidated::new_from_json_bytes(
        decoded_request,
    )?;

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

    Ok(RadixConnectMobileRequest::new(
        session_id,
        origin,
        public_key,
        identity_public_key,
        dapp_definition_address,
        signature,
        request,
    ))
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
mod tests {
    use hex::ToHex;
    use rand::random;

    use super::*;

    const SESSION_ID: &str = "8feeb997-81ff-46ec-a679-e7e697b01601";
    const ORIGIN: &str = "https://radquest-dev.rdx-works-main.extratools.works";
    const PUBLIC_KEY: &str =
        "df856ce8d64bd59aca1bec03584513c49e635f350ff6a312021854d62d54171c";
    const IDENTITY_PUBLIC_KEY: &str =
        "2e39af5c6905bde9825cd7451b0b6361664ac3a111fcdd10334e5fab6ced9fdf";
    const REQUEST: &str = "eyJpdGVtcyI6eyJkaXNjcmltaW5hdG9yIjoiYXV0aG9yaXplZFJlcXVlc3QiLCJhdXRoIjp7ImRpc2NyaW1pbmF0b3IiOiJsb2dpbldpdGhDaGFsbGVuZ2UiLCJjaGFsbGVuZ2UiOiJkYTNlYzhjMjU5MDliNTc3NTlmMTc2ODkwYWU2Mzg1YTRjZTI4NGRjMTI4ZTU2ODAyNmU2ZjIwMWQ5ZDBlNGFlIn0sIm9uZ29pbmdBY2NvdW50cyI6eyJudW1iZXJPZkFjY291bnRzIjp7InF1YW50aWZpZXIiOiJhdExlYXN0IiwicXVhbnRpdHkiOjF9fSwicmVzZXQiOnsiYWNjb3VudHMiOnRydWUsInBlcnNvbmFEYXRhIjpmYWxzZX19LCJpbnRlcmFjdGlvbklkIjoiNzdjZmIzOGYtNWIxMS00YThmLWJlNWEtMzk4NTBiZWQ4M2FkIiwibWV0YWRhdGEiOnsidmVyc2lvbiI6MiwiZEFwcERlZmluaXRpb25BZGRyZXNzIjoiYWNjb3VudF90ZHhfMl8xMngzcm43dHFxcW0zd2d1ejZrbWc1Znk3c2FmOHY5Mmx0NXh1d2duNmtnaDh6YWVqbGY4MGNlIiwibmV0d29ya0lkIjoyLCJvcmlnaW4iOiJodHRwczovL3JhZHF1ZXN0LWRldi5yZHgtd29ya3MtbWFpbi5leHRyYXRvb2xzLndvcmtzIn19";
    const DAPP_DEFINITION_ADDRESS: &str =
        "account_tdx_2_12x3rn7tqqqm3wguz6kmg5fy7saf8v92lt5xuwgn6kgh8zaejlf80ce";
    const SIGNATURE: &str = "884f1ce51dd815c527a31caf77cb2af1c683c41769f3b96e2dc6ef6bd7f786d8db0c48119585a4b98a6b74848402e8f86e33bb3e8de2dceb8338d707df3b6a03";

    fn build_base_url() -> String {
        format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        )
    }

    #[test]
    fn parse_url_into_request() {
        let connect_url = build_base_url();
        let result = parse_mobile_connect_request(connect_url);

        let expected_interaction = DappToWalletInteractionUnvalidated::new(
            "77cfb38f-5b11-4a8f-be5a-39850bed83ad".parse().unwrap(),
            DappToWalletInteractionItems::AuthorizedRequest(
                DappToWalletInteractionAuthorizedRequestItems::new(
                    DappToWalletInteractionAuthRequestItem::LoginWithChallenge(
                        DappToWalletInteractionAuthLoginWithChallengeRequestItem::new(
                            DappToWalletInteractionAuthChallengeNonce(
                                Exactly32Bytes::from_hex("da3ec8c25909b57759f176890ae6385a4ce284dc128e568026e6f201d9d0e4ae").unwrap()
                            )
                        )
                    ),
                    Some(DappToWalletInteractionResetRequestItem::new(true, false)),
                    Some(
                        DappToWalletInteractionAccountsRequestItem {
                            number_of_accounts: RequestedQuantity {
                                quantifier: RequestedNumberQuantifier::AtLeast,
                                quantity: 1,
                            },
                            challenge: None,
                        },
                    ),
                    None,
                    None,
                    None,
                )
            ),
            DappToWalletInteractionMetadataUnvalidated::new(
                WalletInteractionVersion(2),
                NetworkID::Stokenet,
                DappOrigin::from(ORIGIN),
                DAPP_DEFINITION_ADDRESS,
            )
        );
        let expected_request = RadixConnectMobileRequest::new(
            SESSION_ID.parse().unwrap(),
            DappOrigin::from(ORIGIN),
            KeyAgreementPublicKey::from_hex(PUBLIC_KEY.to_owned()).unwrap(),
            Ed25519PublicKey::from_hex(IDENTITY_PUBLIC_KEY.to_owned()).unwrap(),
            DAPP_DEFINITION_ADDRESS.parse().unwrap(),
            SIGNATURE.parse().unwrap(),
            expected_interaction,
        );

        pretty_assertions::assert_eq!(result, Ok(expected_request));

        let parsed_request = result.unwrap();
        let expected_interaction =
            DappToWalletInteractionUnvalidated::new_from_json_bytes(
                URL_SAFE_NO_PAD.decode(REQUEST).unwrap(),
            )
            .unwrap();
        assert_eq!(parsed_request.interaction, expected_interaction);
        assert_eq!(parsed_request.session_id.to_string(), SESSION_ID);
        assert_eq!(parsed_request.origin.to_string(), ORIGIN);
        assert_eq!(parsed_request.public_key.to_hex(), PUBLIC_KEY);
        assert_eq!(
            parsed_request.identity_public_key.to_hex(),
            IDENTITY_PUBLIC_KEY
        );
        assert_eq!(
            parsed_request.dapp_definition_address.to_string(),
            DAPP_DEFINITION_ADDRESS
        );
        assert_eq!(parsed_request.signature.to_hex(), SIGNATURE);
    }

    #[test]
    fn parse_url_with_invalid_scheme() {
        let invalid_scheme_url = format!(
            "invalidScheme://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(invalid_scheme_url.clone());

        assert_ne!(
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
        let missing_param_url = format!(
            "{}://?sessionId={}&origin={}",
            APP_SCHEME, SESSION_ID, ORIGIN
        );
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
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: invalid_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_with_invalid_request() {
        let invalid_request_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request=invalid_request&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(invalid_request_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestFormat)
        );
    }

    #[test]
    fn parse_url_with_invalid_session_id() {
        let invalid_session_id_url = format!(
            "{}://?sessionId=invalid_session_id&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(invalid_session_id_url);
        assert!(matches!(
            result,
            Err(CommonError::RadixConnectMobileInvalidSessionID { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_public_key() {
        let invalid_public_key_url = format!(
            "{}://?sessionId={}&origin={}&publicKey=invalid_public_key&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(invalid_public_key_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidKeyAgreementPublicKeyFromHex { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_signature() {
        let invalid_signature_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature=invalid_signature&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(invalid_signature_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidEd25519SignatureFromString { .. })
        ));
    }

    #[test]
    fn parse_url_with_invalid_identity_key() {
        let invalid_identity_key_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity=invalid_identity_key",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE
        );
        let result = parse_mobile_connect_request(invalid_identity_key_url);
        assert!(matches!(
            result,
            Err(CommonError::InvalidEd25519PublicKeyFromString { .. })
        ));
    }

    #[test]
    fn parse_url_missing_session_id() {
        let missing_session_id_url = format!(
            "{}://?origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(&missing_session_id_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_session_id_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_missing_origin() {
        let missing_origin_url = format!(
            "{}://?sessionId={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(&missing_origin_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_origin_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_missing_interaction() {
        let missing_interaction_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&dAppDefinitionAddress={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, DAPP_DEFINITION_ADDRESS, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(&missing_interaction_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_interaction_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_missing_dapp_definition_address() {
        let missing_dapp_definition_address_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result =
            parse_mobile_connect_request(&missing_dapp_definition_address_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_dapp_definition_address_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_missing_signature() {
        let missing_signature_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, IDENTITY_PUBLIC_KEY
        );
        let result = parse_mobile_connect_request(&missing_signature_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_signature_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_missing_identity() {
        let missing_identity_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, DAPP_DEFINITION_ADDRESS, SIGNATURE
        );
        let result = parse_mobile_connect_request(&missing_identity_url);
        assert_eq!(
            result,
            Err(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: missing_identity_url.to_owned(),
            })
        );
    }

    #[test]
    fn parse_url_with_invalid_dapp_definition_address() {
        let invalid_dapp_definition_address_url = format!(
            "{}://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress=invalid_dapp_definition_address&signature={}&identity={}",
            APP_SCHEME, SESSION_ID, ORIGIN, PUBLIC_KEY, REQUEST, SIGNATURE, IDENTITY_PUBLIC_KEY
        );
        let result =
            parse_mobile_connect_request(&invalid_dapp_definition_address_url);
        assert!(matches!(
            result,
            Err(CommonError::FailedToDecodeAddressFromBech32 { .. })
        ));
    }
}
