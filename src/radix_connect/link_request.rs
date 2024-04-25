use crate::prelude::*;

use super::session_id;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct LinkRequest {
    pub origin: Url,
    pub session_id: SessionID,
}

impl LinkRequest {
    pub fn new(origin: Url, session_id: SessionID) -> Self {
        Self { origin, session_id }
    }

    pub(crate) fn try_with_origin_and_session_id(
        origin: impl AsRef<str>,
        session_id: impl AsRef<str>,
    ) -> Result<Self> {
        let origin = parse_url(origin.as_ref()).map_err(|_| {
            CommonError::RadixConnectMobileInvalidOrigin {
                bad_value: origin.as_ref().to_owned(),
            }
        })?;
        let session_id =
            SessionID::from_str(session_id.as_ref()).map_err(|_| {
                CommonError::RadixConnectMobileInvalidSessionID {
                    bad_value: session_id.as_ref().to_owned(),
                }
            })?;
        Ok(LinkRequest::new(origin, session_id))
    }
}

impl HasSampleValues for LinkRequest {
    fn sample() -> Self {
        Self {
            origin: parse_url("radix://app1").unwrap(),
            session_id: SessionID::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            origin: parse_url("radix://app2").unwrap(),
            session_id: SessionID::sample_other(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = LinkRequest;

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
    fn test_new() {
        let origin = parse_url("radix://app").unwrap();
        let session_id = SessionID::sample();
        let sut = SUT::new(origin.clone(), session_id.clone());
        assert_eq!(sut.origin, origin);
        assert_eq!(sut.session_id, session_id);
    }
}
