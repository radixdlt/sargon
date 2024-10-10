use crate::prelude::*;

#[derive(
    Clone,
    
    
    PartialEq,
    Eq,
    Hash,
    strum::EnumString,
    strum::Display,
    uniffi::Enum,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NetworkMethod {
    Post,
    Get,
    Head,
}

#[uniffi::export]
pub fn new_network_method_sample() -> NetworkMethod {
    NetworkMethod::sample()
}

#[uniffi::export]
pub fn new_network_method_sample_other() -> NetworkMethod {
    NetworkMethod::sample_other()
}

#[uniffi::export]
pub fn network_method_to_string(method: &NetworkMethod) -> String {
    method.to_string()
}

