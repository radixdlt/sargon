use crate::prelude::*;
use sargon::RadixConnectMobileSessionRequest as InternalRadixConnectMobileSessionRequest;

json_data_convertible!(RadixConnectMobileSessionRequest);

/// The request received from the dApp that needs to be handled.
#[derive(PartialEq, Clone, InternalConversion, uniffi::Record)]
pub struct RadixConnectMobileSessionRequest {
    /// The id of the session established with the dApp.
    /// Needs to be passed back by the Host as to know which session to respond to.
    pub session_id: SessionID,

    /// The interaction received from the dApp.
    pub interaction: DappToWalletInteractionUnvalidated,

    /// The origin of the dApp.
    pub origin: DappOrigin,

    /// Whether the origin requires validation.
    pub origin_requires_validation: bool,
}

impl From<InternalRadixConnectMobileSessionRequest>
    for RadixConnectMobileSessionRequest
{
    fn from(value: InternalRadixConnectMobileSessionRequest) -> Self {
        Self {
            session_id: value.session_id.into(),
            interaction: value.interaction.into(),
            origin: value.origin.into(),
            origin_requires_validation: value.origin_requires_validation,
        }
    }
}

impl Into<InternalRadixConnectMobileSessionRequest>
    for RadixConnectMobileSessionRequest
{
    fn into(self) -> InternalRadixConnectMobileSessionRequest {
        InternalRadixConnectMobileSessionRequest {
            session_id: self.session_id.into(),
            interaction: self.interaction.into(),
            origin: self.origin.into(),
            origin_requires_validation: self.origin_requires_validation,
        }
    }
}

#[uniffi::export]
pub fn new_radix_connect_mobile_session_request_sample(
) -> RadixConnectMobileSessionRequest {
    InternalRadixConnectMobileSessionRequest::sample().into()
}

#[uniffi::export]
pub fn new_radix_connect_mobile_session_request_sample_other(
) -> RadixConnectMobileSessionRequest {
    InternalRadixConnectMobileSessionRequest::sample_other().into()
}
