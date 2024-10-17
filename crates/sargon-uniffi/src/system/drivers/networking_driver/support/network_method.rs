use crate::prelude::*;
use sargon::NetworkMethod as InternalNetworkMethod;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum NetworkMethod {
    Post,
    Get,
    Head,
}

#[uniffi::export]
pub fn new_network_method_sample() -> NetworkMethod {
    InternalNetworkMethod::sample().into()
}

#[uniffi::export]
pub fn new_network_method_sample_other() -> NetworkMethod {
    InternalNetworkMethod::sample_other().into()
}

#[uniffi::export]
pub fn network_method_to_string(method: &NetworkMethod) -> String {
    method.into_internal().to_string()
}
