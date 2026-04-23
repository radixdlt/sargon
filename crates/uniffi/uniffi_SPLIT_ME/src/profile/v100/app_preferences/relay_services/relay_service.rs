use crate::prelude::*;
use sargon::Identifiable;
use sargon::RelayService as InternalRelayService;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct RelayService {
    pub name: String,
    pub url: RelayServiceUrl,
}

#[uniffi::export]
pub fn new_relay_service_sample() -> RelayService {
    InternalRelayService::sample().into()
}

#[uniffi::export]
pub fn new_relay_service_sample_other() -> RelayService {
    InternalRelayService::sample_other().into()
}

#[uniffi::export]
pub fn relay_service_id(relay_service: &RelayService) -> RelayServiceUrl {
    relay_service.url.clone()
}
