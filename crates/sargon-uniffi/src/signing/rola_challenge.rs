use crate::prelude::*;
use sargon::RolaChallenge as InternalRolaChallenge;

#[derive(Debug, Clone, PartialEq, InternalConversion, uniffi::Record)]
pub struct RolaChallenge {
    pub payload: BagOfBytes,
}

#[uniffi::export]
pub fn rola_challenge_get_hash(rola_challenge: &RolaChallenge) -> Hash {
    rola_challenge.into_internal().hash().into()
}

#[uniffi::export]
pub fn new_rola_challenge_sample() -> RolaChallenge {
    InternalRolaChallenge::sample().into()
}

#[uniffi::export]
pub fn new_rola_challenge_sample_other() -> RolaChallenge {
    InternalRolaChallenge::sample_other().into()
}
