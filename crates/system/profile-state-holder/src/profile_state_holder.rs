use crate::prelude::*;

#[derive(Debug)]
pub struct ProfileStateHolder {
    // This is pub(crate) for testing purposes only, i.e. causing the RwLock to be poisoned.
    pub(crate) state: RwLock<ProfileState>,
}

impl ProfileStateHolder {
    pub fn new(state: ProfileState) -> Self {
        Self {
            state: RwLock::new(state),
        }
    }
}

impl ProfileStateHolder {
    /// Clone the profile and return it.
    pub fn profile(&self) -> Result<Profile> {
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

    /// Returns the non-hidden personas on the current network, empty if no personas
    /// on the network
    pub fn personas_on_current_network(&self) -> Result<Personas> {
        self.try_access_profile_with(|p| p.personas_on_current_network())
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

    pub fn entity_by_address(
        &self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona> {
        self.try_access_profile_with(|p| p.entity_by_address(entity_address))
    }

    pub fn entity_by_access_controller_address(
        &self,
        address_of_access_controller: AccessControllerAddress,
    ) -> Result<AccountOrPersona> {
        self.try_access_profile_with(|p| {
            p.entity_by_access_controller_address(address_of_access_controller)
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

    /// Looks up the persona by identity address, returns Err if the persona is
    /// unknown, will return a hidden persona if queried for.
    pub fn persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<Persona> {
        self.try_access_profile_with(|p| p.persona_by_address(address))
    }

    pub fn access_profile_with<T, F>(&self, access: F) -> Result<T>
    where
        F: Fn(&Profile) -> T,
    {
        let guard = self.state.read().expect(
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

    pub fn try_access_profile_with<T, F>(&self, access: F) -> Result<T>
    where
        F: Fn(&Profile) -> Result<T>,
    {
        let guard = self.state.read().expect(
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
    pub fn replace_profile_state_with(
        &self,
        profile_state: ProfileState,
    ) -> Result<()> {
        let mut lock = self.state.write().expect(
            "Stop execution due to the profile state lock being poisoned",
        );
        Self::diagnostics_for_factor_instances_valid(&profile_state);
        *lock = profile_state;
        Ok(())
    }

    pub(crate) fn diagnostics_for_factor_instances_valid(
        profile_state: &ProfileState,
    ) {
        let Some(profile) = profile_state.as_loaded() else {
            return;
        };
        profile.diagnostics_for_factor_instances_valid();
    }

    /// Updates the in-memory profile held by this `ProfileStateHolder`, you might
    /// wanna also persist the change in the `SargonOS` by saving it to secure
    /// storage.
    pub fn update_profile_with<F, R>(&self, mutate: F) -> Result<R>
    where
        F: Fn(&mut Profile) -> Result<R>,
    {
        let mut guard = self.state.write().expect(
            "Stop execution due to the profile state lock being poisoned",
        );

        let state = &mut *guard;

        match state {
            ProfileState::Loaded(ref mut profile) => {
                mutate(profile).inspect(|_| {
                    profile.diagnostics_for_factor_instances_valid();
                })
            }
            _ => Err(CommonError::ProfileStateNotLoaded {
                current_state: state.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_none_profile_state_holder() {
        let state = ProfileState::None;
        assert_eq!(
            ProfileStateHolder::new(state.clone())
                .state
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
                .state
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
                .state
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

        let first_mainnet_account: Account = state_holder
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

        let account_address = first_mainnet_account.clone().address;
        let expected_name =
            first_mainnet_account.clone().display_name.value() + "01234";
        for i in 0..5 {
            let state_holder_clone = Arc::clone(&state_holder);
            let handle = thread::spawn(move || {
                let _write_lock =
                    state_holder_clone.update_profile_with(|profile| {
                        profile.networks.try_update_with(
                            &NetworkID::Mainnet,
                            |network| {
                                let _res = network.accounts.try_update_with(
                                    &account_address,
                                    |account| {
                                        let display_name =
                                            account.display_name.value();
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
                    .value()
            })
            .unwrap();

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
