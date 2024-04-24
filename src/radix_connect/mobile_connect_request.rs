use crate::prelude::*;

#[derive(Debug, uniffi::Enum)]
pub enum MobileConnectRequest {
    Link(LinkRequest),
    DappInteraction(DappRequest),
}

#[derive(Debug, uniffi::Record)]
pub struct LinkRequest {
    pub origin: Url,
    pub session_id: SessionID,
}

#[derive(Debug, uniffi::Record)]
pub struct DappRequest {
    pub interaction_id: WalletInteractionId,
    pub session_id: SessionID,
}
