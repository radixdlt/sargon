use crate::prelude::*;

pub trait ProfileHasAnyAccountOnAnyNetwork {
    /// If the user has **any** accounts on any network at all, including hidden
    /// accounts. This can be used by host devices to prompt user to create their
    /// first account or not, e.g. if user starts app after fresh install, the
    /// SargonOS will create an "empty" Profile and BDFS and save it, before user
    /// has had the chance to create their first account. If the user force quits
    /// the app and then restart it, the app can still prompt user to create their
    /// first account - as if no force-restart happened.
    fn has_any_account_on_any_network(&self) -> bool;

    fn contains_entity_by_address(
        &self,
        entity_address: &AddressOfAccountOrPersona,
    ) -> bool;
}

impl ProfileHasAnyAccountOnAnyNetwork for Profile {
    /// If the user has **any** accounts on any network at all, including hidden
    /// accounts. This can be used by host devices to prompt user to create their
    /// first account or not, e.g. if user starts app after fresh install, the
    /// SargonOS will create an "empty" Profile and BDFS and save it, before user
    /// has had the chance to create their first account. If the user force quits
    /// the app and then restart it, the app can still prompt user to create their
    /// first account - as if no force-restart happened.
    fn has_any_account_on_any_network(&self) -> bool {
        self.networks.iter().any(|n| !n.accounts.is_empty())
    }

    fn contains_entity_by_address(
        &self,
        entity_address: &AddressOfAccountOrPersona,
    ) -> bool {
        self.networks.iter().any(|n: ProfileNetwork| {
            n.contains_entity_by_address(entity_address)
        })
    }
}
pub trait EntityOnNetworkHandling {
    fn get_account(&self, address: &AccountAddress) -> Option<Account>;
    fn get_persona(&self, address: &IdentityAddress) -> Option<Persona>;
    fn update_entities<E: IsEntity>(
        &mut self,
        updated_entities: IdentifiedVecOf<E>,
    ) -> Result<()>;
    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()>;

    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account);

    fn hide_account(&mut self, account_address: &AccountAddress) -> bool;

    fn tombstone_account(&mut self, account_address: &AccountAddress) -> bool;

    /// Tombstones the accounts
    fn tombstone_accounts(&mut self, account_addresses: &Vec<AccountAddress>) {
        for account_address in account_addresses {
            self.tombstone_account(account_address);
        }
    }

    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona);
}

impl EntityOnNetworkHandling for ProfileNetworks {
    fn get_account(&self, address: &AccountAddress) -> Option<Account> {
        self.get_id(address.network_id())
            .and_then(|n| n.accounts.get_id(address))
            .cloned()
    }

    fn get_persona(&self, address: &IdentityAddress) -> Option<Persona> {
        self.get_id(address.network_id())
            .and_then(|n| n.personas.get_id(address))
            .cloned()
    }

    fn update_entities<E: IsEntity>(
        &mut self,
        updated_entities: IdentifiedVecOf<E>,
    ) -> Result<()> {
        self.update_entities_erased(
            updated_entities.into_iter().map(Into::into).collect(),
        )
    }

    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()> {
        let network =
            updated_entities.assert_elements_not_empty_and_on_same_network()?;
        self.try_try_update_with(&network, |n| {
            n.update_entities_erased(updated_entities.clone())
        })
    }

    /// Returns a clone of the updated account if found, else None.
    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mut mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        self.update_with(address.network_id(), |n| {
            _ = n.update_account(address, |a| mutate(a))
        });
        self.get_account(address)
    }

    /// Hides the account associated with the `account_address`
    fn hide_account(&mut self, account_address: &AccountAddress) -> bool {
        self.update_with(account_address.network_id(), |n| {
            n.hide_account(account_address);
        })
    }

    /// Tombstones the account associated with the `account_address`
    fn tombstone_account(&mut self, account_address: &AccountAddress) -> bool {
        self.update_with(account_address.network_id(), |n| {
            n.tombstone_account(account_address);
        })
    }

    /// Returns a clone of the updated persona if found, else None.
    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mut mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona),
    {
        self.update_with(address.network_id(), |n| {
            _ = n.update_persona(address, |a| mutate(a))
        });
        self.get_persona(address)
    }
}

pub trait ProfileNetworkEntitiesUpdating {
    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()>;

    fn update_entities<E: IsEntity>(
        &mut self,
        updated_entities: IdentifiedVecOf<E>,
    ) -> Result<()> {
        self.update_entities_erased(
            updated_entities.into_iter().map(Into::into).collect(),
        )
    }
}

impl ProfileNetworkEntitiesUpdating for ProfileNetwork {
    fn update_entities_erased(
        &mut self,
        updated_entities: IdentifiedVecOf<AccountOrPersona>,
    ) -> Result<()> {
        for entity in updated_entities {
            match entity {
                AccountOrPersona::AccountEntity(account) => self
                    .accounts
                    .try_update_with(&account.id(), |a| *a = account.clone())
                    .map_err(|_| CommonError::UnknownAccount),
                AccountOrPersona::PersonaEntity(persona) => self
                    .personas
                    .try_update_with(&persona.id(), |p| *p = persona.clone())
                    .map_err(|_| CommonError::UnknownPersona),
            }?;
        }
        Ok(())
    }
}

pub trait ProfileNetworkAccountUpdating {
    /// Returns a clone of the updated account if found, else None.
    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account);

    /// Hides the account associated with the `account_address`
    fn hide_account(
        &mut self,
        account_address: &AccountAddress,
    ) -> Option<Account>;

    /// Tombstones the account associated with the `account_address`
    fn tombstone_account(
        &mut self,
        account_address: &AccountAddress,
    ) -> Option<Account>;
}

impl ProfileNetworkAccountUpdating for ProfileNetwork {
    /// Returns a clone of the updated account if found, else None.
    fn update_account<F>(
        &mut self,
        address: &AccountAddress,
        mutate: F,
    ) -> Option<Account>
    where
        F: FnMut(&mut Account),
    {
        if self.accounts.update_with(address, mutate) {
            self.accounts.get_id(address).cloned()
        } else {
            None
        }
    }

    /// Hides the account associated with the `account_address`
    fn hide_account(
        &mut self,
        account_address: &AccountAddress,
    ) -> Option<Account> {
        let account = self.update_account(account_address, |account| {
            account.mark_as_hidden();
        });
        self.authorized_dapps
            .remove_referenced_account(account_address);
        account
    }

    /// Tombstones the account associated with the `account_address`
    fn tombstone_account(
        &mut self,
        account_address: &AccountAddress,
    ) -> Option<Account> {
        let account = self.update_account(account_address, |account| {
            account.mark_as_tombstoned();
        });
        self.authorized_dapps
            .remove_referenced_account(account_address);
        account
    }
}

pub trait ProfileNetworkPersonaUpdating {
    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona);
}

impl ProfileNetworkPersonaUpdating for ProfileNetwork {
    /// Returns a clone of the updated persona if found, else None.
    fn update_persona<F>(
        &mut self,
        address: &IdentityAddress,
        mutate: F,
    ) -> Option<Persona>
    where
        F: FnMut(&mut Persona),
    {
        if self.personas.update_with(address, mutate) {
            self.personas.get_id(address).cloned()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod profile_network_tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = ProfileNetworks;

    #[test]
    fn update_account_unknown_account() {
        let mut sut = SUT::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample_mainnet_carol().address;
        assert_eq!(
            sut.get_id(id).unwrap().accounts.get_id(account_address),
            None
        );

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, SUT::sample());
    }

    #[test]
    fn update_account_unknown_network() {
        let mut sut = SUT::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample_nebunet().address;
        assert_eq!(
            sut.get_id(id).unwrap().accounts.get_id(account_address),
            None
        );

        assert!(sut
            .update_account(&account_address, |a| {
                a.display_name = DisplayName::new("will fail").unwrap()
            })
            .is_none());

        // Assert unchanged
        assert_eq!(sut, SUT::sample());
    }

    #[test]
    fn update_account() {
        let mut sut = SUT::sample();
        let id = &NetworkID::Mainnet;
        let account_address = Account::sample().address;
        assert_eq!(
            sut.get_id(id)
                .unwrap()
                .accounts
                .get_id(account_address)
                .unwrap()
                .display_name
                .value(),
            "Alice"
        );

        sut.update_account(&account_address, |a| {
            a.display_name = DisplayName::new("Stella").unwrap()
        });

        assert_eq!(
            sut.get_id(id)
                .unwrap()
                .accounts
                .get_id(account_address)
                .unwrap()
                .display_name
                .value(),
            "Stella"
        );
    }
}

#[cfg(test)]
mod profile_tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Profile;

    #[test]
    fn test_empty_profile_has_any_account_on_any_network_is_false() {
        let sut =
            SUT::new(Mnemonic::sample(), HostId::sample(), HostInfo::sample());
        assert!(!sut.has_any_account_on_any_network());
    }

    #[test]
    fn test_sample_profile_has_any_account_on_any_network() {
        assert!(SUT::sample().has_any_account_on_any_network());
        assert!(SUT::sample_other().has_any_account_on_any_network());
    }

    #[test]
    fn new_from_bdfs_with_accounts() {
        let accounts = Accounts::sample_mainnet();
        let profile = SUT::from_device_factor_source(
            DeviceFactorSource::sample(),
            HostId::sample(),
            HostInfo::sample(),
            Some(accounts),
        );

        assert!(profile.has_any_account_on_any_network())
    }
}
