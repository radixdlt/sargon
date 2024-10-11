use crate::prelude::*;
use sargon::NetworkRequest as InternalNetworkRequest;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

impl From<InternalNetworkRequest> for NetworkRequest {
    fn from(value: InternalNetworkRequest) -> Self {
        Self {
            url: value.url.into(),
            method: value.method.into(),
            headers: value.headers.into_hash_map(),
            body: value.body.into(),
        }
    }
}

impl Into<InternalNetworkRequest> for NetworkRequest {
    fn into(self) -> InternalNetworkRequest {
        InternalNetworkRequest {
            url: self.url.into(),
            method: self.method.into(),
            headers: self.headers.into_internal_hash_map(),
            body: self.body.into(),
        }
    }
}

#[uniffi::export]
pub fn new_network_request_sample() -> NetworkRequest {
    InternalNetworkRequest::sample().into()
}

#[uniffi::export]
pub fn new_network_request_sample_other() -> NetworkRequest {
    InternalNetworkRequest::sample_other().into()
}
