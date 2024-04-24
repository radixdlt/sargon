use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Enum)]
pub enum MobileConnectRequest {
    Link(LinkRequest),
    DappInteraction(DappRequest),
}

impl FromStr for MobileConnectRequest {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        parse_mobile_connect_request(s.to_owned())
    }
}

#[uniffi::export]
pub fn new_mobile_connect_request(url: String) -> Result<MobileConnectRequest> {
    MobileConnectRequest::from_str(url.as_str())
}

impl HasSampleValues for MobileConnectRequest {
    fn sample() -> Self {
        MobileConnectRequest::Link(LinkRequest::sample())
    }

    fn sample_other() -> Self {
        MobileConnectRequest::DappInteraction(DappRequest::sample())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MobileConnectRequest;

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
    fn test_new_mobile_connect_request() {
        let connect_url = "https://d1rxdfxrfmemlj.cloudfront.net/?sessionId=123&origin=radix%3A%2F%2Fapp";
        assert!(new_mobile_connect_request(connect_url.to_owned()).is_ok());
    }
}
