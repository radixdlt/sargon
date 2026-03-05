use crate::prelude::*;
use sargon::P2PStunServer as InternalP2PStunServer;
use sargon::P2PTurnServer as InternalP2PTurnServer;

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct P2PStunServer {
    pub urls: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Record)]
pub struct P2PTurnServer {
    pub urls: Vec<String>,
    pub username: Option<String>,
    pub credential: Option<String>,
}

#[uniffi::export]
pub fn new_p2p_stun_server_sample() -> P2PStunServer {
    InternalP2PStunServer::sample().into()
}

#[uniffi::export]
pub fn new_p2p_stun_server_sample_other() -> P2PStunServer {
    InternalP2PStunServer::sample_other().into()
}

#[uniffi::export]
pub fn new_p2p_turn_server_sample() -> P2PTurnServer {
    InternalP2PTurnServer::sample().into()
}

#[uniffi::export]
pub fn new_p2p_turn_server_sample_other() -> P2PTurnServer {
    InternalP2PTurnServer::sample_other().into()
}
