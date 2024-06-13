use crate::prelude::*;

use super::super::session::session_id::SessionID;

json_data_convertible!(RadixConnectMobileLinkRequest);

#[derive(Debug, Serialize, Deserialize, PartialEq, uniffi::Record)]
pub struct RadixConnectMobileLinkRequest {
    pub origin: Url,
    pub session_id: SessionID,
    pub public_key: KeyAgreementPublicKey,
    pub browser: String,
}

impl RadixConnectMobileLinkRequest {
    pub fn new(
        origin: Url,
        session_id: SessionID,
        public_key: KeyAgreementPublicKey,
        browser: String,
    ) -> Self {
        Self {
            origin,
            session_id,
            public_key,
            browser,
        }
    }

    pub(crate) fn new_from_raw_components(
        origin: impl AsRef<str>,
        session_id: impl AsRef<str>,
        public_key: impl AsRef<str>,
        browser: impl AsRef<str>,
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
        let public_key =
            KeyAgreementPublicKey::from_hex(public_key.as_ref().into())?;
        Ok(RadixConnectMobileLinkRequest::new(
            origin,
            session_id,
            public_key,
            browser.as_ref().into(),
        ))
    }
}

impl HasSampleValues for RadixConnectMobileLinkRequest {
    fn sample() -> Self {
        RadixConnectMobileLinkRequest::new(
            parse_url("http://app1.com").unwrap(),
            SessionID::sample(),
            KeyAgreementPublicKey::sample(),
            "Chrome".into(),
        )
    }

    fn sample_other() -> Self {
        RadixConnectMobileLinkRequest::new(
            parse_url("http://app2.com").unwrap(),
            SessionID::sample_other(),
            KeyAgreementPublicKey::sample_other(),
            "Safari".into(),
        )
    }
}

#[uniffi::export]
pub fn new_radix_connect_mobile_link_request_sample(
) -> RadixConnectMobileLinkRequest {
    RadixConnectMobileLinkRequest::sample()
}

#[uniffi::export]
pub fn new_radix_connect_mobile_link_request_sample_other(
) -> RadixConnectMobileLinkRequest {
    RadixConnectMobileLinkRequest::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileLinkRequest;

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

#[cfg(test)]
mod uniffi_tests {
    use crate::prelude::*;

    #[test]
    fn sample_values() {
        assert_eq!(
            new_radix_connect_mobile_link_request_sample(),
            RadixConnectMobileLinkRequest::sample()
        );
        assert_eq!(
            new_radix_connect_mobile_link_request_sample_other(),
            RadixConnectMobileLinkRequest::sample_other()
        );
    }
}
