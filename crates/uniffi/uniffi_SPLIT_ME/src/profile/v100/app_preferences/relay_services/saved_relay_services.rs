use crate::prelude::*;
use sargon::SavedRelayServices as InternalSavedRelayServices;

decl_vec_samples_for!(RelayServices, RelayService);

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SavedRelayServices {
    pub current: RelayService,
    pub other: Vec<RelayService>,
}

#[uniffi::export]
pub fn new_saved_relay_services(current: RelayService) -> SavedRelayServices {
    InternalSavedRelayServices::new(current.into()).into()
}

#[uniffi::export]
pub fn new_saved_relay_services_default() -> SavedRelayServices {
    InternalSavedRelayServices::default().into()
}

#[uniffi::export]
pub fn new_saved_relay_services_sample() -> SavedRelayServices {
    InternalSavedRelayServices::sample().into()
}

#[uniffi::export]
pub fn new_saved_relay_services_sample_other() -> SavedRelayServices {
    InternalSavedRelayServices::sample_other().into()
}

#[uniffi::export]
pub fn saved_relay_services_get_all_elements(
    services: &SavedRelayServices,
) -> Vec<RelayService> {
    services.into_internal().all().into_type()
}
