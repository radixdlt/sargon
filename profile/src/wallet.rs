use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{Account, AccountAddress, CommonError, DisplayName, HasPlaceholder, Profile};

#[derive(Debug, uniffi::Object)]
pub struct Wallet {
    profile: RwLock<Profile>,
}

#[uniffi::export]
impl Wallet {
    #[uniffi::constructor]
    pub fn new() -> Self {
        let profile = Profile::placeholder();
        Self {
            profile: RwLock::new(profile),
        }
    }

    /// Tries to clone the profile (snapshot) and return it.
    pub fn profile_snapshot(&self) -> Result<Profile, CommonError> {
        self.read(|p| p.clone())
    }

    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: String,
    ) -> Result<Account, CommonError> {
        DisplayName::new(to.as_str())
            .and_then(|new_name| {
                self.write(
                    |mut p| p.update_account(&address, |a| a.display_name = new_name.to_owned())
                )
            })
            .and_then(|r| r.ok_or_else(|| CommonError::AccountOnWrongNetwork))
    }
}

impl Wallet {
    fn read<T: Clone, F>(&self, access: F) -> Result<T, CommonError>
    where
        F: Fn(RwLockReadGuard<'_, Profile>) -> T,
    {
        self.profile
            .try_read()
            .map(access)
            .map_err(|_| CommonError::FailedToReadProfileUnableToAcquireLock)
    }

    fn write<F, R>(&self, mutate: F) -> Result<R, CommonError>
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> R,
    {
        self.profile
            .try_write()
            .map(mutate)
            .map_err(|_| CommonError::FailedToModifyProfileUnableToAcquireLock)
    }
}
