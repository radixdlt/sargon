use crate::prelude::*;
use std::{
    borrow::Borrow,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug, uniffi::Object)]
#[allow(dead_code)]
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

impl ProfileHolder {
    /// Clone the profile and return it.
    pub(super) fn profile(&self) -> Profile {
        self.access_profile_with(|p| p.clone())
    }

    pub(super) fn access_profile_with<T, F>(&self, access: F) -> T
    where
        F: Fn(RwLockReadGuard<'_, Profile>) -> T,
    {
        self.profile
            .try_read()
            .map(access)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }

    pub(super) fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> Result<R>,
    {
        self.profile
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)
            .and_then(mutate)
    }
}
