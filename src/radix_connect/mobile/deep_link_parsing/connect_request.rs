use super::*;
use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Enum)]
pub enum RadixConnectMobileConnectRequest {
    Link(RadixConnectMobileLinkRequest),
    DappInteraction(RadixConnectMobileDappRequest),
}

impl FromStr for RadixConnectMobileConnectRequest {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self> {
        parse_mobile_connect_request(s)
    }
}

impl HasSampleValues for RadixConnectMobileConnectRequest {
    fn sample() -> Self {
        RadixConnectMobileConnectRequest::Link(
            RadixConnectMobileLinkRequest::sample(),
        )
    }

    fn sample_other() -> Self {
        RadixConnectMobileConnectRequest::DappInteraction(
            RadixConnectMobileDappRequest::sample(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileConnectRequest;

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
        let uuid = Uuid::new_v4().to_string();
        let connect_url = format!("https://d1rxdfxrfmemlj.cloudfront.net/?sessionId={}&origin=radix%3A%2F%2Fapp", uuid);
        assert!(RadixConnectMobileConnectRequest::from_str(
            connect_url.as_str()
        )
        .is_ok());
    }
}
