use crate::prelude::*;
use std::sync::RwLock;


#[derive(Debug, uniffi::Object)]
pub struct ProfileClient {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile: RwLock<Profile>,
}
