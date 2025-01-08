use crate::prelude::*;
use sargon::NetworkRequest as InternalNetworkRequest;

#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

#[uniffi::export]
pub fn new_network_request_sample() -> NetworkRequest {
    InternalNetworkRequest::sample().into()
}

#[uniffi::export]
pub fn new_network_request_sample_other() -> NetworkRequest {
    InternalNetworkRequest::sample_other().into()
}
