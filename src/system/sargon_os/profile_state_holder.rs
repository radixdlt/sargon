use crate::prelude::*;
use std::{borrow::Borrow, sync::RwLock};

#[derive(Debug, Clone, PartialEq, derive_more::Display, uniffi::Enum)]
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
        self.access_profile_with(|p| p.current_network().clone())
    }

    /// Returns the non-hidden accounts on the current network, empty if no accounts
    /// on the network
    pub fn accounts_on_current_network(&self) -> Result<Accounts> {
        self.access_profile_with(|p| p.accounts_on_current_network())
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
        self.access_profile_with(|p| {
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
        let guard = self
            .profile_state
            .try_read()
            .expect("Implementing hosts should not read and write Profile from multiple threads.");

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
        let guard = self
            .profile_state
            .try_read()
            .expect("Implementing hosts should not read and write Profile from multiple threads.");

        let state = &*guard;
        match state {
            ProfileState::Loaded(profile) => Ok(profile),
            _ => Err(CommonError::ProfileStateNotLoaded {
                current_state: state.to_string(),
            }),
        }
        .and_then(access)
    }

    /// Sets the profile held by this `ProfileStateHolder` to `profile`.
    pub(super) fn replace_profile_state_with(
        &self,
        profile_state: ProfileState,
    ) -> Result<()> {
        let mut lock = self
            .profile_state
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)?;

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
        self.profile_state
            .try_write()
            .map_err(|_| CommonError::UnableToAcquireWriteLockForProfile)
            .and_then(|mut guard| {
                let state = &mut *guard;

                match state {
                    ProfileState::Loaded(ref mut profile) => mutate(profile),
                    _ => Err(CommonError::ProfileStateNotLoaded {
                        current_state: state.to_string(),
                    }),
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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
}
