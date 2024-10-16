use crate::prelude::*;
use std::{borrow::Borrow, sync::RwLock};

#[derive(
    Debug, Clone, PartialEq, EnumAsInner, derive_more::Display, uniffi::Enum,
)]
#[allow(clippy::large_enum_variant)]
pub enum ProfileState {
    /// When no profile exists in secure storage when OS is booted.
    None,

    /// When the profile snapshot retrieved from secure storage failed to convert into a
    /// valid Profile.
    Incompatible(CommonError),

    /// When a valid 'Profile' exists. This can either happen when the os boots, or a profile is
    /// restored, or the user creates a new profile.
    #[display("Loaded: {}", _0.id())]
    Loaded(Profile),
}

#[derive(Debug, uniffi::Object)]
#[allow(dead_code)]
pub struct ProfileStateHolder {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) profile_state: RwLock<ProfileState>,
}

impl ProfileStateHolder {
    pub fn new(profile_state: ProfileState) -> Self {
        Self {
            profile_state: RwLock::new(profile_state),
        }
    }
}

impl ProfileStateHolder {
    /// Clone the profile and return it.
    pub(super) fn profile(&self) -> Result<Profile> {
        self.access_profile_with(|p| p.clone())
    }

    pub fn current_network_id(&self) -> Result<NetworkID> {
        self.access_profile_with(|p| p.current_network_id())
    }

    pub fn current_gateway(&self) -> Result<Gateway> {
        self.access_profile_with(|p| p.current_gateway().clone())
    }

    pub fn gateways(&self) -> Result<SavedGateways> {
        self.access_profile_with(|p| p.app_preferences.gateways.clone())
    }

    pub fn current_network(&self) -> Result<ProfileNetwork> {
        self.try_access_profile_with(|p| p.current_network().cloned())
    }

    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Result<Accounts> {
        self.try_access_profile_with(|p| p.accounts_on_current_network())
    }

    /// Returns all the SecurityStructuresOfFactorSources,
    /// by trying to map FactorSourceID level -> FactorSource Level
    pub fn security_structures_of_factor_sources(
        &self,
    ) -> Result<SecurityStructuresOfFactorSources> {
        self.try_access_profile_with(|p| {
            p.security_structures_of_factor_sources()
        })
    }

    /// Returns the non-hidden accounts on the current network as `AccountForDisplay`
    pub fn accounts_for_display_on_current_network(
        &self,
    ) -> Result<AccountsForDisplay> {
        self.try_access_profile_with(|p| {
            p.accounts_for_display_on_current_network()
        })
    }

    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.try_access_profile_with(|p| p.account_by_address(address))
    }

    pub fn access_profile_with<T, F>(&self, access: F) -> Result<T>
    where
        F: Fn(&Profile) -> T,
    {
        let guard = self.profile_state.read().expect(
            "Stop execution due to the profile state lock being poisoned",
        );

        let state = &*guard;
        match state {
            ProfileState::Loaded(profile) => Ok(profile),
            _ => Err(CommonError::ProfileStateNotLoaded {
                current_state: state.to_string(),
            }),
        }
        .map(access)
    }

    pub(super) fn try_access_profile_with<T, F>(&self, access: F) -> Result<T>
    where
        F: Fn(&Profile) -> Result<T>,
    {
        let guard = self.profile_state.read().expect(
            "Stop execution due to the profile state lock being poisoned",
        );

        let state = &*guard;
        match state {
            ProfileState::Loaded(profile) => Ok(profile),
            _ => Err(CommonError::ProfileStateNotLoaded {
                current_state: state.to_string(),
            }),
        }
        .and_then(access)
    }

    /// Sets the `ProfileState` held by this `ProfileStateHolder` to the latest `profile_state`.
    pub(super) fn replace_profile_state_with(
        &self,
        profile_state: ProfileState,
    ) -> Result<()> {
        let mut lock = self.profile_state.write().expect(
            "Stop execution due to the profile state lock being poisoned",
        );

        *lock = profile_state;
        Ok(())
    }

    /// Updates the in-memory profile held by this `ProfileStateHolder`, you might
    /// wanna also persist the change in the `SargonOS` by saving it to secure
    /// storage.
    pub(super) fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(&mut Profile) -> Result<R>,
    {
        let mut guard = self.profile_state.write().expect(
            "Stop execution due to the profile state lock being poisoned",
        );

        let state = &mut *guard;

        match state {
            ProfileState::Loaded(ref mut profile) => mutate(profile),
            _ => Err(CommonError::ProfileStateNotLoaded {
                current_state: state.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::sync::{Arc, RwLock};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_none_profile_state_holder() {
        let state = ProfileState::None;
        assert_eq!(
            ProfileStateHolder::new(state.clone())
                .profile_state
                .try_read()
                .unwrap()
                .to_owned(),
            state
        )
    }

    #[test]
    fn test_new_incompatible_profile_state_holder() {
        let state =
            ProfileState::Incompatible(CommonError::InvalidISO8601String {
                bad_value: "-".to_owned(),
            });
        assert_eq!(
            ProfileStateHolder::new(state.clone())
                .profile_state
                .try_read()
                .unwrap()
                .to_owned(),
            state
        )
    }

    #[test]
    fn test_new_loaded_profile_state_holder() {
        let state = ProfileState::Loaded(Profile::sample());
        assert_eq!(
            ProfileStateHolder::new(state.clone())
                .profile_state
                .try_read()
                .unwrap()
                .to_owned(),
            state,
        )
    }

    #[test]
    fn test_concurrent_access_read_after_write() {
        let state = ProfileState::Loaded(Profile::sample());
        let sut = ProfileStateHolder::new(state.clone());
        let state_holder = Arc::new(sut);

        let state_holder_clone = Arc::clone(&state_holder);

        // Spawn a thread that acquires a write lock
        let handle = thread::spawn(move || {
            let _write_lock =
                state_holder_clone.update_profile_with(|profile| {
                    profile.networks.try_update_with(
                        &NetworkID::Mainnet,
                        |network| {
                            let _res = network.accounts.try_insert_unique(
                                Account::sample_mainnet_carol(),
                            );
                        },
                    )
                });
            thread::sleep(Duration::from_millis(200));
        });

        // Give the other thread time to acquire the write lock
        thread::sleep(Duration::from_millis(100));

        let mainnet_accounts = state_holder.current_network().unwrap().accounts;

        handle.join().unwrap(); // Wait for the thread to finish

        let mut expected_accounts = Accounts::sample_mainnet();
        expected_accounts.insert(Account::sample_mainnet_carol());
        pretty_assertions::assert_eq!(mainnet_accounts, expected_accounts)
    }

    #[test]
    fn test_concurrent_access_writes_order_is_preserved() {
        let profile = Profile::sample();
        let state = ProfileState::Loaded(profile);
        let sut = ProfileStateHolder::new(state.clone());
        let state_holder = Arc::new(sut);

        let first_mainnet_account = state_holder
            .access_profile_with(|profile| {
                profile
                    .networks
                    .first()
                    .unwrap()
                    .accounts
                    .first()
                    .unwrap()
                    .clone()
            })
            .unwrap();

        let mut handles = vec![];

        for i in 0..5 {
            let state_holder_clone = Arc::clone(&state_holder);
            let handle = thread::spawn(move || {
                let _write_lock =
                    state_holder_clone.update_profile_with(|profile| {
                        profile.networks.try_update_with(
                            &NetworkID::Mainnet,
                            |network| {
                                let _res = network.accounts.try_update_with(
                                    &first_mainnet_account.address,
                                    |account| {
                                        let display_name =
                                            account.display_name.value.clone();
                                        account.display_name = DisplayName::new(
                                            display_name
                                                + i.to_string().as_str(),
                                        )
                                        .unwrap()
                                    },
                                );
                            },
                        )
                    });
                // Hold the lock for a while to simulate a long-running write operation
                thread::sleep(Duration::from_millis(200));
            });
            thread::sleep(Duration::from_millis(100));

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let result_name = state_holder
            .access_profile_with(|profile| {
                profile
                    .networks
                    .first()
                    .unwrap()
                    .accounts
                    .first()
                    .unwrap()
                    .display_name
                    .value
                    .clone()
            })
            .unwrap();

        let expected_name = first_mainnet_account.display_name.value + "01234";

        pretty_assertions::assert_eq!(expected_name, result_name)
    }

    #[test]
    #[should_panic]
    fn test_concurrent_access_poisoned_lock_panics() {
        let state = ProfileState::Loaded(Profile::sample());
        let sut = ProfileStateHolder::new(state.clone());
        let state_holder = Arc::new(sut);

        let state_holder_clone = Arc::clone(&state_holder);

        // Spawn a thread that acquires a write lock
        let handle = thread::spawn(move || {
            let _write_lock =
                state_holder_clone.update_profile_with(|profile| {
                    profile.networks.try_update_with(
                        &NetworkID::Mainnet,
                        |network| {
                            network
                                .accounts
                                .try_insert_unique(
                                    Account::sample_mainnet_carol(),
                                )
                                .unwrap();
                            panic!("Simulate panic in thread");
                        },
                    )
                });
        });

        let _ = handle.join(); // Wait for the thread to finish

        state_holder.current_network().unwrap();
    }
}
