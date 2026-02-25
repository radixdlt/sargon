use crate::prelude::*;
use sargon::P2PIceServer as InternalP2PIceServer;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct P2PIceServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

#[uniffi::export]
pub fn new_p2p_ice_server_sample() -> P2PIceServer {
    InternalP2PIceServer::sample().into()
}

#[uniffi::export]
pub fn new_p2p_ice_server_sample_other() -> P2PIceServer {
    InternalP2PIceServer::sample_other().into()
}
