use crate::prelude::*;
use sargon::NetworkRequest as InternalNetworkRequest;

#[derive(Clone, Debug, PartialEq, Eq,  uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: BagOfBytes,
}

impl From<InternalNetworkRequest> for NetworkRequest {
    fn from(value: InternalNetworkRequest) -> Self {
        unimplemented!()
    }
}

use crate::prelude::*;

#[uniffi::export]
pub fn new_network_request_sample() -> NetworkRequest {
    NetworkRequest::sample()
}

#[uniffi::export]
pub fn new_network_request_sample_other() -> NetworkRequest {
    NetworkRequest::sample_other()
}

