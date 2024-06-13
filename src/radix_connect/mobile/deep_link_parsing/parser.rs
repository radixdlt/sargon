use crate::prelude::*;
use base64::engine::general_purpose::URL_SAFE;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use url::form_urlencoded;
use url::Url;

use super::*;

const CONNECT_URL_PARAM_SESSION_ID: &str = "sessionId";
const CONNECT_URL_PARAM_ORIGIN: &str = "origin";
const CONNECT_URL_PARAM_INTERACTION_ID: &str = "interactionId";
const CONNECT_URL_PARAM_INTERACTION: &str = "walletInteraction";
const CONNECT_URL_PARAM_PUBLIC_KEY: &str = "publicKey";
const CONNECT_URL_PARAM_BROWSER: &str = "browser";
const CONNECT_URL: &str = "https://d1rxdfxrfmemlj.cloudfront.net";
const APP_SCHEME: &str = "radixwallet";

pub fn parse_mobile_connect_request(
    url: impl AsRef<str>,
) -> Result<RadixConnectMobileConnectRequest> {
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
    let session_id_string = query_parameters
        .get(CONNECT_URL_PARAM_SESSION_ID)
        .ok_or(CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        })?;
    let browser = query_parameters.get(CONNECT_URL_PARAM_BROWSER).ok_or(
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        },
    )?;

    let Some(origin) = query_parameters.get(CONNECT_URL_PARAM_ORIGIN) else {
        let Some(raw_interaction) =
            query_parameters.get(CONNECT_URL_PARAM_INTERACTION)
        else {
            return query_parameters.get(CONNECT_URL_PARAM_INTERACTION_ID)
                .ok_or(
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned(),
            })
                .and_then(|interaction_id| {
                RadixConnectMobileDappRequest::try_with_interaction_id_and_session_id(interaction_id, session_id_string)
                })
                .map(RadixConnectMobileConnectRequest::DappInteraction);
        };

        let decoded = URL_SAFE_NO_PAD.decode(raw_interaction.as_str()).unwrap();
        let request =
            DappToWalletInteractionUnvalidated::new_from_json_bytes(decoded)
                .unwrap();
        return RadixConnectMobileDappRequestContained::try_with_request_and_session_id(request, session_id_string)
            .map(RadixConnectMobileConnectRequest::DappInteractionContained);
    };

    let public_key = query_parameters.get(CONNECT_URL_PARAM_PUBLIC_KEY).ok_or(
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.to_owned(),
        },
    )?;

    RadixConnectMobileLinkRequest::new_from_raw_components(
        origin,
        session_id_string,
        public_key,
        browser,
    )
    .map(RadixConnectMobileConnectRequest::Link)
}

#[cfg(test)]
mod tests {
    use base64::prelude::BASE64_URL_SAFE_NO_PAD;

    use super::*;

    #[test]
    fn parse_url_into_link_request_origin() {
        // let x = "eyJpdGVtcyI6eyJkaXNjcmltaW5hdG9yIjoiYXV0aG9yaXplZFJlcXVlc3QiLCJhdXToIjp7ImRpc2NyaW1pbmF0b3IiOiJsb2dpbldpdGhDaGFsbGVuZ2UiLCJjaGFsbGVuZ2UiOiI0NjNhNWFkYjU1ZjhkMTcxODIxMWVjZGUyYjJkMDllNDU5ZWM2ZWQwNjIxMmQ4OGIwOThiODVlNzBmODA4ZDBhIn0sInJlc2V0Ijp7ImFjY291bnRzIjpmYWxzZSwicGVyc29uYURhdGEiOmZhbHNlfX0sImludGVyYWN0aW9uSWQiOiI0NGY1ODBlOC0wMmQzLTQ2YjgtYTMzNS1hOTZiYjAwYTE0ZmQiLCJtZXRhZGF0YSI6eyJ2ZXJzaW9uIjoyLCJkQXBwRGVmaW5pdGlvbkFkZHJlc3MiOiJhY2NvdW50X3RkeF8yXzEyeWY5Z2Q1M3lmZXA3YTY2OWZ2MnQzd203bno5emVlendkMDRuMDJhANDMza2VyOHZ6YTZyaGUiLCJuZXR3b3JrSWQiOjIsIm9yaWdpbiI6Imh0dHBzOi8vZDIzNjczM3E5azg2M2IuY2xvdWRmcm9udC5uZXQifX0";
        // let decoded = URL_SAFE_NO_PAD.decode(x).unwrap();
        // let y = DappToWalletInteractionUnvalidated::new_from_json_bytes(decoded)
        //         .unwrap();
        let origin = parse_url("radix://app").unwrap();
        let session_id = SessionID::sample();
        let public_key =
            KeyAgreementPrivateKey::generate().unwrap().public_key();
        let browser = "chrome".to_string();
        let connect_url = CONNECT_URL.to_owned()
            + format!("/?sessionId={}&origin=radix%3A%2F%2Fapp&publicKey={}&browser=chrome", session_id.to_string(), public_key.to_hex())
                .as_str();

        let result = parse_mobile_connect_request(connect_url);
        let expected_result = RadixConnectMobileConnectRequest::Link(
            RadixConnectMobileLinkRequest::new(
                origin, session_id, public_key, browser,
            ),
        );

        pretty_assertions::assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn parse_url_app_scheme_into_link_request_origin() {
        let origin = parse_url("radix://app").unwrap();
        let session_id = SessionID::sample();
        let public_key =
            KeyAgreementPrivateKey::generate().unwrap().public_key();
        let browser = "chrome".to_string();
        let connect_url = APP_SCHEME.to_owned()
            + format!("://?sessionId={}&origin=radix%3A%2F%2Fapp&publicKey={}&browser=chrome", session_id.to_string(), public_key.to_hex())
                .as_str();

        pretty_assertions::assert_eq!(
            parse_url(&connect_url).unwrap().scheme(),
            APP_SCHEME
        );

        let result = parse_mobile_connect_request(connect_url);
        let expected_result = RadixConnectMobileConnectRequest::Link(
            RadixConnectMobileLinkRequest::new(
                origin, session_id, public_key, browser,
            ),
        );

        pretty_assertions::assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn parse_url_wrong_session_id() {
        let interaction_id = Uuid::new_v4().to_string();
        let connect_url = CONNECT_URL.to_owned()
            + format!(
                "/?sessionId=123&interactionId={}&browser=chrome",
                interaction_id
            )
            .as_str();

        let err = parse_mobile_connect_request(connect_url.clone())
            .err()
            .unwrap();
        pretty_assertions::assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidSessionID {
                bad_value: "123".to_owned()
            }
        );
    }

    #[test]
    fn parse_url_into_dapp_interaction() {
        let session_id = Uuid::new_v4().to_string();
        let interaction_id = Uuid::new_v4().to_string();
        let url = CONNECT_URL.to_owned()
            + format!(
                "/?sessionId={}&interactionId={}&browser=chrome",
                session_id, interaction_id
            )
            .as_str();
        let result = parse_mobile_connect_request(url);
        assert!(result.is_ok());
        match result.unwrap() {
            RadixConnectMobileConnectRequest::DappInteraction(dapp_request) => {
                assert_eq!(dapp_request.session_id.0.to_string(), session_id);
                assert_eq!(
                    dapp_request.interaction_id.0.to_string(),
                    interaction_id
                );
            }
            _ => {
                panic!("Expected DappRequest");
            }
        }
    }

    #[test]
    fn url_does_not_match_expected() {
        let url = "https://example.com";
        let err = parse_mobile_connect_request(url).err().unwrap();
        pretty_assertions::assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned()
            }
        );
    }

    #[test]
    fn url_invalid() {
        let url = "http/invalid_url";
        let err = parse_mobile_connect_request(url).err().unwrap();
        pretty_assertions::assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned()
            }
        );
    }

    #[test]
    fn url_with_invalid_origin() {
        let session_id = Uuid::new_v4().to_string();
        let public_key =
            KeyAgreementPrivateKey::generate().unwrap().public_key();
        let connect_url = CONNECT_URL.to_owned()
            + format!(
                "/?sessionId={}&origin=invalid&publicKey={}&browser=chrome",
                session_id,
                public_key.to_hex()
            )
            .as_str();
        let err = parse_mobile_connect_request(connect_url).err().unwrap();
        pretty_assertions::assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidOrigin {
                bad_value: "invalid".to_owned()
            }
        );
    }

    #[test]
    fn url_does_not_match_any_request() {
        let url = "https://d1rxdfxrfmemlj.cloudfront.net/?invalid=1";
        let err = parse_mobile_connect_request(url).err().unwrap();
        pretty_assertions::assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned()
            }
        );
    }
}
