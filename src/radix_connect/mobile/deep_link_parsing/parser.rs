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
const CONNECT_URL: &str = "https://d1rxdfxrfmemlj.cloudfront.net";
const APP_SCHEME: &str = "radixwallet";

pub fn parse_mobile_connect_request(
    url: impl AsRef<str>,
) -> Result<RadixConnectMobileRequest> {
    let url = url.as_ref();
    let connect_url = parse_url(CONNECT_URL).unwrap();
    let parsed_url = parse_url(url).map_err(|_| {
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        }
    })?;

    if !(parsed_url.host_str() == connect_url.host_str()
        && parsed_url.scheme() == connect_url.scheme()
        || parsed_url.scheme() == APP_SCHEME)
    {
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

    let decoded_request =
        URL_SAFE_NO_PAD.decode(request_string.as_str()).unwrap();
    let request = DappToWalletInteractionUnvalidated::new_from_json_bytes(
        decoded_request,
    )
    .unwrap();

    let dapp_definition_address_string = get_key(
        url,
        &query_parameters,
        CONNECT_URL_PARAM_DAPP_DEFINITION_ADDRESS,
    )?;
    let signature_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_SIGNATURE)?;
    let identity_public_key_string =
        get_key(url, &query_parameters, CONNECT_URL_PARAM_IDENTITY_KEY)?;

    let origin = parse_url(origin_string.clone()).map_err(|_| {
        CommonError::RadixConnectMobileInvalidOrigin {
            bad_value: origin_string,
        }
    })?;
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse_url_into_request() {
//         let session_id = "8feeb997-81ff-46ec-a679-e7e697b01601";
//         let origin = "https://77c1c1f54ef4.ngrok.app";
//         let public_key =
//             "df856ce8d64bd59aca1bec03584513c49e635f350ff6a312021854d62d54171c";
//         let identity_public_key =
//             "2e39af5c6905bde9825cd7451b0b6361664ac3a111fcdd10334e5fab6ced9fdf";
//         let request = "eyJpdGVtcyI6eyJkaXNjcmltaW5hdG9yIjoiYXV0aG9yaXplZFJlcXVlc3QiLCJhdXToIjp7ImRpc2NyaW1pbmF0b3IiOiJsb2dpbldpdGhDaGFsbGVuZ2UiLCJjaGFsbGVuZ2UiOiIxMTQ4ODFlZDk3YTdlYmVlYTdlNmZhMjM4YzMwANDBiZGJhYTk3YTkzZGI2OTMzY2Q5YjI4YjJmNGUyOGU0MjUwIn0sInJlc2V0Ijp7ImFjY291bnRzIjpmYWxzZSwicGVyc29uYURhdGEiOmZhbHNlfX0sImludGVyYWN0aW9uSWQiOiI0YTJhYTRkOC01ZGIwLTRjMGQtYjI1Yy05NGY0YTk0ZTU5MmMiLCJtZXRhZGF0YSI6eyJ2ZXJzaW9uIjoyLCJkQXBwRGVmaW5pdGlvbkFkZHJlc3MiOiJhY2NvdW50X3RkeF8yXzEyeWY5Z2Q1M3lmZXA3YTY2OWZ2MnQzd203bno5emVlendkMDRuMDJhANDMza2VyOHZ6YTZyaGUiLCJuZXR3b3JrSWQiOjIsIm9yaWdpbiI6Imh0dHBzOi8vNzdjMWMxZjU0ZWY0Lm5ncm9rLmFwcCJ9fQ";
//         let dapp_definition_address = "account_tdx_2_12yf9gd53yfep7a669fv2t3wm7nz9zeezwd04n02a433ker8vza6rhe";
//         let signature = "884f1ce51dd815c527a31caf77cb2af1c683c41769f3b96e2dc6ef6bd7f786d8db0c48119585a4b98a6b74848402e8f86e33bb3e8de2dceb8338d707df3b6a03";

//         let connect_url = APP_SCHEME.to_owned()
//             + format!(
//             "://?sessionId={}&origin={}&publicKey={}&request={}&dAppDefinitionAddress={}&signature={}&identity={}",
//             session_id,
//             origin,
//             public_key,
//             request,
//             dapp_definition_address,
//             signature,
//            identity_public_key,
//         )
//                 .as_str();

//         let result = parse_mobile_connect_request(connect_url);

//         let expected_interaction = DappToWalletInteractionUnvalidated::new(
//             "4a2aa4d8-5db0-4c0d-b25c-94f4a94e592c".parse().unwrap(),
//             DappToWalletInteractionItems::AuthorizedRequest(
//                 DappToWalletInteractionAuthorizedRequestItems::new(
//                     DappToWalletInteractionAuthRequestItem::LoginWithChallenge(
//                         DappToWalletInteractionAuthLoginWithChallengeRequestItem::new(
//                             DappToWalletInteractionAuthChallengeNonce(
//                                 Exactly32Bytes::from_hex("114881ed97a7ebeea7e6fa238c3040bdbaa97a93db6933cd9b28b2f4e28e4250").unwrap()
//                             )
//                         )
//                     ),
//                     Some(DappToWalletInteractionResetRequestItem::new(false, false)),
//                     None,
//                     None,
//                     None,
//                     None,
//                 )
//             ),
//             DappToWalletInteractionMetadataUnvalidated::new(
//                 WalletInteractionVersion(2),
//                 NetworkID::Stokenet,
//                 Url::parse(&origin).unwrap(),
//                 dapp_definition_address.to_owned(),
//             )
//         );
//         let expected_request = RadixConnectMobileRequest::new(
//             session_id.parse().unwrap(),
//             Url::parse(&origin).unwrap(),
//             KeyAgreementPublicKey::from_hex(public_key.to_owned()).unwrap(),
//             Ed25519PublicKey::from_hex(identity_public_key.to_owned()).unwrap(),
//             dapp_definition_address.parse().unwrap(),
//             signature.parse().unwrap(),
//             expected_interaction,
//         );

//         // let message = [
//         //     "C".as_bytes(),
//         //     expected_request.request.interaction_id.0.as_bytes(),
//         //     "69".as_bytes(),
//         //     expected_request
//         //         .dapp_definition_address
//         //         .to_string()
//         //         .as_bytes(),
//         //     expected_request.origin.to_string().as_bytes(),
//         // ]
//         // .concat();

//         // let hash = hash_of(message);

//         // let is_valid_signature = expected_request
//         //     .identity_public_key
//         //     .is_valid_signature_for_hash(&expected_request.signature, &hash);

//         // pretty_assertions::assert_eq!(is_valid_signature, true);

//         pretty_assertions::assert_eq!(result, Ok(expected_request));
//     }
// }
