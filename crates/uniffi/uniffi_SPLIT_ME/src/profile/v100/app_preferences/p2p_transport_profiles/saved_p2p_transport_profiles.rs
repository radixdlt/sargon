use crate::prelude::*;
use sargon::SavedP2PTransportProfiles as InternalSavedP2PTransportProfiles;

decl_vec_samples_for!(P2PTransportProfiles, P2PTransportProfile);

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct SavedP2PTransportProfiles {
    pub current: P2PTransportProfile,
    pub other: Vec<P2PTransportProfile>,
}

#[uniffi::export]
pub fn new_saved_p2p_transport_profiles(
    current: P2PTransportProfile,
) -> SavedP2PTransportProfiles {
    InternalSavedP2PTransportProfiles::new(current.into()).into()
}

#[uniffi::export]
pub fn new_saved_p2p_transport_profiles_default() -> SavedP2PTransportProfiles {
    InternalSavedP2PTransportProfiles::default().into()
}

#[uniffi::export]
pub fn new_saved_p2p_transport_profiles_sample() -> SavedP2PTransportProfiles {
    InternalSavedP2PTransportProfiles::sample().into()
}

#[uniffi::export]
pub fn new_saved_p2p_transport_profiles_sample_other(
) -> SavedP2PTransportProfiles {
    InternalSavedP2PTransportProfiles::sample_other().into()
}

#[uniffi::export]
pub fn saved_p2p_transport_profiles_get_all_elements(
    profiles: &SavedP2PTransportProfiles,
) -> Vec<P2PTransportProfile> {
    profiles
        .into_internal()
        .all_available_for_selection()
        .into_type()
}
