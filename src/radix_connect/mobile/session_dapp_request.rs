use crate::prelude::*;

json_data_convertible!(RadixConnectMobileSessionRequest);

#[derive(Debug, PartialEq, uniffi::Record, Serialize, Deserialize)]
pub struct RadixConnectMobileSessionRequest {
    pub session_id: SessionID,
    pub interaction: DappToWalletInteractionUnvalidated,
    pub origin: DappOrigin,
    pub origin_requires_validation: bool,
}

impl RadixConnectMobileSessionRequest {
    pub fn new(
        session_id: impl Into<SessionID>,
        interaction: DappToWalletInteractionUnvalidated,
        origin: DappOrigin,
        origin_requires_validation: bool,
    ) -> Self {
        Self {
            session_id: session_id.into(),
            interaction,
            origin,
            origin_requires_validation,
        }
    }
}

impl HasSampleValues for RadixConnectMobileSessionRequest {
    fn sample() -> Self {
        Self::new(
            SessionID::sample(),
            DappToWalletInteractionUnvalidated::sample(),
            DappOrigin::sample(),
            true,
        )
    }

    fn sample_other() -> Self {
        Self::new(
            SessionID::sample_other(),
            DappToWalletInteractionUnvalidated::sample_other(),
            DappOrigin::sample_other(),
            false,
        )
    }
}

#[uniffi::export]
pub fn new_radix_connect_mobile_session_request_sample(
) -> RadixConnectMobileSessionRequest {
    RadixConnectMobileSessionRequest::sample()
}

#[uniffi::export]
pub fn new_radix_connect_mobile_session_request_sample_other(
) -> RadixConnectMobileSessionRequest {
    RadixConnectMobileSessionRequest::sample_other()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RadixConnectMobileSessionRequest;

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
            new_radix_connect_mobile_session_request_sample(),
            RadixConnectMobileSessionRequest::sample()
        );
        assert_eq!(
            new_radix_connect_mobile_session_request_sample_other(),
            RadixConnectMobileSessionRequest::sample_other()
        );
    }
}
