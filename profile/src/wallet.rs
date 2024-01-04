use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{Account, AccountAddress, CommonError, DisplayName, Profile};

#[derive(Debug, uniffi::Object)]
pub struct Wallet {
    profile: RwLock<Profile>,
}

#[uniffi::export]
impl Wallet {
    #[uniffi::constructor]
    pub fn new(profile: Profile) -> Self {
        Self {
            profile: RwLock::new(profile),
        }
    }

    /// Cone the profile (snapshot) and return it.
    pub fn profile_snapshot(&self) -> Profile {
        self.read(|p| p.clone())
    }

    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account, CommonError> {
        self.write(|mut p| p.update_account(&address, |a| a.display_name = to.to_owned()))
            .ok_or_else(|| CommonError::UnknownAccount)
    }
}

impl Wallet {
    fn read<T: Clone, F>(&self, access: F) -> T
    where
        F: Fn(RwLockReadGuard<'_, Profile>) -> T,
    {
        self.profile
            .try_read()
            .map(access)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }

    fn write<F, R>(&self, mutate: F) -> R
    where
        F: Fn(RwLockWriteGuard<'_, Profile>) -> R,
    {
        self.profile
            .try_write()
            .map(mutate)
            .expect("Implementing Wallet clients should not read and write Profile from Wallet from multiple threads.")
    }
}

#[cfg(test)]
mod tests {
    use crate::{DisplayName, HasPlaceholder, Profile};

    use super::Wallet;

    #[test]
    fn read_header() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone());
        wallet.read(|p| assert_eq!(p.header, profile.header))
    }

    #[test]
    fn take_snapshot() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone());
        assert_eq!(wallet.profile_snapshot(), profile)
    }

    #[test]
    fn change_display_name_of_accounts() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone());
        let account = wallet.read(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        assert!(wallet
            .change_name_of_account(account.address, DisplayName::new("Stella").unwrap())
            .is_ok());
        wallet.read(|p| assert_eq!(p.networks[0].accounts[0].display_name.value, "Stella"));
    }
}
