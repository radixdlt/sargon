use crate::prelude::*;
use sargon::NetworkMethod as InternalNetworkMethod;

#[derive(
    Clone, 
    PartialEq,
    Eq,
    Hash,
    InternalConversion,
    uniffi::Enum,
)]
pub enum NetworkMethod {
    Post,
    Get,
    Head,
}

impl From<InternalNetworkMethod> for NetworkMethod {
    fn from(value: InternalNetworkMethod) -> Self {
        match value {
            InternalNetworkMethod::Post => NetworkMethod::Post,
            InternalNetworkMethod::Get => NetworkMethod::Get,
            InternalNetworkMethod::Head => NetworkMethod::Head,
        }
    }
}

impl Into<InternalNetworkMethod> for NetworkMethod {
    fn into(self) -> InternalNetworkMethod {
        match self {
            NetworkMethod::Post => InternalNetworkMethod::Post,
            NetworkMethod::Get => InternalNetworkMethod::Get,
            NetworkMethod::Head => InternalNetworkMethod::Head,
        }
    }
}

#[uniffi::export]
pub fn new_network_method_sample() -> NetworkMethod {
    NetworkMethod::sample().into()
}

#[uniffi::export]
pub fn new_network_method_sample_other() -> NetworkMethod {
    NetworkMethod::sample_other().into()
}

#[uniffi::export]
pub fn network_method_to_string(method: &NetworkMethod) -> String {
    method.into_internal().to_string()
}

