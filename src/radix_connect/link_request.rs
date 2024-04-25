use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct LinkRequest {
    pub origin: Url,
    pub session_id: SessionID,
}

impl LinkRequest {
    pub(crate) fn try_with_origin_and_session_id(
        origin: impl AsRef<str>,
        session_id: SessionID,
    ) -> Result<Self> {
        let origin = Url::parse(origin.as_ref()).map_err(|_| {
            CommonError::RadixConnectMobileInvalidOrigin {
                bad_value: origin.as_ref().to_owned(),
            }
        })?;
        Ok(Self { origin, session_id })
    }
}

impl HasSampleValues for LinkRequest {
    fn sample() -> Self {
        Self {
            origin: Url::parse("radix://app").unwrap(),
            session_id: SessionID::sample(),
        }
    }

    fn sample_other() -> Self {
        Self {
            origin: Url::parse("radix://app").unwrap(),
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
}
