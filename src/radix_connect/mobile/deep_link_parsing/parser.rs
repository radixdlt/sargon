use crate::prelude::*;
use url::form_urlencoded;
use url::Url;

use super::*;

const CONNECT_URL_PARAM_SESSION_ID: &str = "sessionId";
const CONNECT_URL_PARAM_ORIGIN: &str = "origin";
const CONNECT_URL_PARAM_INTERACTION_ID: &str = "interactionId";
const CONNECT_URL: &str = "https://d1rxdfxrfmemlj.cloudfront.net";

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
    if parsed_url.host_str() != connect_url.host_str()
        || parsed_url.scheme() != connect_url.scheme()
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
    let Some(origin) = query_parameters.get(CONNECT_URL_PARAM_ORIGIN) else {
        return query_parameters
            .get(CONNECT_URL_PARAM_INTERACTION_ID)
            .ok_or(CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned(),
            })
            .and_then(|interaction_id| {
                RadixConnectMobileDappRequest::try_with_interaction_id_and_session_id(
                    interaction_id,
                    session_id_string,
                )
            })
            .map(RadixConnectMobileConnectRequest::DappInteraction);
    };

    RadixConnectMobileLinkRequest::try_with_origin_and_session_id(
        origin,
        session_id_string,
    )
    .map(RadixConnectMobileConnectRequest::Link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_into_link_request_origin() {
        let session_id = Uuid::new_v4().to_string();
        let connect_url = CONNECT_URL.to_owned()
            + format!("/?sessionId={}&origin=radix%3A%2F%2Fapp", session_id)
                .as_str();
        let result = parse_mobile_connect_request(connect_url);
        assert!(result.is_ok());
        match result.unwrap() {
            RadixConnectMobileConnectRequest::Link(link_request) => {
                assert_eq!(link_request.session_id.0.to_string(), session_id);
                assert_eq!(
                    link_request.origin,
                    parse_url("radix://app").unwrap()
                );
            }
            _ => {
                panic!("Expected LinkRequest");
            }
        }
    }

    #[test]
    fn parse_url_wrong_session_id() {
        let connect_url =
            CONNECT_URL.to_owned() + "/?sessionId=123&origin=radix%3A%2F%2Fapp";
        let err = parse_mobile_connect_request(connect_url.clone())
            .err()
            .unwrap();
        assert_eq!(
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
                "/?sessionId={}&interactionId={}",
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
        assert_eq!(
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
        assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned()
            }
        );
    }

    #[test]
    fn url_with_invalid_origin() {
        let session_id = Uuid::new_v4().to_string();
        let connect_url = CONNECT_URL.to_owned()
            + format!("/?sessionId={}&origin=invalid", session_id).as_str();
        let err = parse_mobile_connect_request(connect_url).err().unwrap();
        assert_eq!(
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
        assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl {
                bad_value: url.to_owned()
            }
        );
    }
}
