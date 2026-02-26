use crate::prelude::*;
use sargon::Identifiable;
use sargon::P2PTransportProfile as InternalP2PTransportProfile;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct P2PTransportProfile {
    pub name: String,
    pub signaling_server: String,
    pub ice_servers: Vec<P2PIceServer>,
}

#[uniffi::export]
pub fn new_p2p_transport_profile_sample() -> P2PTransportProfile {
    InternalP2PTransportProfile::sample().into()
}

#[uniffi::export]
pub fn new_p2p_transport_profile_sample_other() -> P2PTransportProfile {
    InternalP2PTransportProfile::sample_other().into()
}

#[uniffi::export]
pub fn p2p_transport_profile_id(profile: &P2PTransportProfile) -> String {
    profile.into_internal().id()
}
