use crate::prelude::*;
use std::sync::RwLock;

#[derive(Debug, uniffi::Object)]
pub struct ProfileHolder {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile: RwLock<Profile>,
}

impl ProfileHolder {
    pub fn new(profile: Profile) -> Self {
        Self {
            profile: RwLock::new(profile),
        }
    }
}

impl From<Profile> for ProfileHolder {
    fn from(value: Profile) -> Self {
        Self::new(value)
    }
}
