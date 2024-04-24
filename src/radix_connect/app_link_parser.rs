use crate::prelude::*;
use url::form_urlencoded;
use url::Url;

const CONNECT_URL_PARAM_SESSION_ID: &str = "sessionId";
const CONNECT_URL_PARAM_ORIGIN: &str = "origin";
const CONNECT_URL_PARAM_INTERACTION_ID: &str = "interactionId";
const CONNECT_URL: &str = "https://d1rxdfxrfmemlj.cloudfront.net";

pub fn parse_mobile_connect_request(
    url: String,
) -> Result<MobileConnectRequest> {
    let connect_url = Url::parse(CONNECT_URL).unwrap();
    let parsed_url = Url::parse(url.as_str()).map_err(|_| {
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.clone(),
        }
    })?;
    if parsed_url.host_str() != connect_url.host_str()
        || parsed_url.scheme() != connect_url.scheme()
    {
        return Err(CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.clone(),
        });
    }

    let query_parameters = parsed_url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();
    let session_id = query_parameters.get(CONNECT_URL_PARAM_SESSION_ID).ok_or(
        CommonError::RadixConnectMobileInvalidRequestUrl {
            bad_value: url.clone(),
        },
    )?;

    match query_parameters.get(CONNECT_URL_PARAM_ORIGIN) {
        Some(origin) => Url::parse(origin)
            .map_err(|_| CommonError::RadixConnectMobileInvalidOrigin {
                bad_value: origin.to_owned(),
            })
            .map(|url| {
                MobileConnectRequest::Link(LinkRequest {
                    origin: url,
                    session_id: SessionID(session_id.to_owned()),
                })
            }),
        None => {
            let interaction_id = query_parameters
                .get(CONNECT_URL_PARAM_INTERACTION_ID)
                .ok_or(CommonError::RadixConnectMobileInvalidRequestUrl {
                    bad_value: url.clone(),
                })?;
            Ok(MobileConnectRequest::DappInteraction(DappRequest {
                session_id: SessionID(session_id.to_owned()),
                interaction_id: WalletInteractionId(interaction_id.to_owned()),
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_into_link_request_origin() {
        let connect_url =
            CONNECT_URL.to_owned() + "/?sessionId=123&origin=radix%3A%2F%2Fapp";
        let result = parse_mobile_connect_request(connect_url);
        assert!(result.is_ok());
        match result.unwrap() {
            MobileConnectRequest::Link(link_request) => {
                assert_eq!(
                    link_request.session_id,
                    SessionID("123".to_owned())
                );
                assert_eq!(
                    link_request.origin,
                    Url::parse("radix://app").unwrap()
                );
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn parse_url_into_dapp_interaction() {
        let url = CONNECT_URL.to_owned() + "/?sessionId=123&interactionId=456";
        let result = parse_mobile_connect_request(url);
        assert!(result.is_ok());
        match result.unwrap() {
            MobileConnectRequest::DappInteraction(dapp_request) => {
                assert_eq!(
                    dapp_request.session_id,
                    SessionID("123".to_owned())
                );
                assert_eq!(
                    dapp_request.interaction_id,
                    WalletInteractionId("456".to_owned())
                );
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn url_does_not_match_expected() {
        let url = String::from("https://example.com");
        let err = parse_mobile_connect_request(url.clone()).err().unwrap();
        assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl { bad_value: url }
        );
    }

    #[test]
    fn url_invalid() {
        let url = String::from("invalid_url");
        let err = parse_mobile_connect_request(url.clone()).err().unwrap();
        assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl { bad_value: url }
        );
    }

    #[test]
    fn url_with_invalid_origin() {
        let connect_url =
            CONNECT_URL.to_owned() + "/?sessionId=123&origin=invalid";
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
        let url =
            String::from("https://d1rxdfxrfmemlj.cloudfront.net/?invalid=1");
        let err = parse_mobile_connect_request(url.clone()).err().unwrap();
        assert_eq!(
            err,
            CommonError::RadixConnectMobileInvalidRequestUrl { bad_value: url }
        );
    }
}
