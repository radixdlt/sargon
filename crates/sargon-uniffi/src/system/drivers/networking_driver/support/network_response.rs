use crate::prelude::*;
use sargon::NetworkResponse as InternalNetworkResponse;

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: BagOfBytes,
}

impl Into<InternalNetworkResponse> for NetworkResponse {
    fn into(self) -> InternalNetworkResponse {
        InternalNetworkResponse {
            status_code: self.status_code,
            body: self.body.into(),
        }
    }
}

impl From<InternalNetworkResponse> for NetworkResponse {
    fn from(value: InternalNetworkResponse) -> Self {
        Self {
            status_code: value.status_code,
            body: value.body.into(),
        }
    }
}