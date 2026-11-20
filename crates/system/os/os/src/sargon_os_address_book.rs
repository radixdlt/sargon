use crate::prelude::*;

impl SargonOS {
    /// Returns all address book entries on the current network.
    pub fn address_book_on_current_network(&self) -> Result<AddressBook> {
        self.profile_state_holder.address_book_on_current_network()
    }

    /// Returns only account address book entries on the current network.
    pub fn account_address_book_on_current_network(
        &self,
    ) -> Result<Vec<AddressBookEntry>> {
        self.profile_state_holder
            .account_address_book_on_current_network()
    }

    /// Looks up the address book entry naming `target`, an address or a
    /// non-fungible, on the current network.
    pub fn address_book_entry(
        &self,
        target: impl Into<AddressBookEntryTarget>,
    ) -> Result<AddressBookEntry> {
        self.profile_state_holder.address_book_entry(target.into())
    }
}

/// Rejects naming something on another network than the current one.
fn ensure_on_current_network(
    profile: &Profile,
    target: &AddressBookEntryTarget,
) -> Result<NetworkID> {
    let current_network = profile.current_network_id();
    if target.network_id() != current_network {
        return Err(CommonError::EntityOnWrongNetwork {
            entity_kind: "AddressBookEntry".to_owned(),
            wrong_network: target.network_id().to_string(),
            expected_network: current_network.to_string(),
        });
    }
    Ok(current_network)
}

impl SargonOS {
    /// Adds a new address book entry naming `target`, an address or a
    /// non-fungible, on the current network.
    pub async fn add_address_book_entry(
        &self,
        target: impl Into<AddressBookEntryTarget>,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<()> {
        let target = target.into();
        self.update_profile_with(|profile| {
            let current_network = ensure_on_current_network(profile, &target)?;

            if let Some(alias) = profile
                .current_network()?
                .address_book
                .get_id(target.clone())
                .map(|entry| entry.name.to_string())
            {
                return Err(CommonError::AddressBookEntryAlreadyExists {
                    alias,
                });
            }

            let entry =
                AddressBookEntry::new(target.clone(), name, note.clone());
            profile.networks.update_with(current_network, |network| {
                network.address_book.add_entry(entry.clone());
            });
            Ok(())
        })
        .await
    }

    /// Updates an existing address book entry on the current network.
    /// Returns false if no entry names `target`.
    pub async fn update_address_book_entry(
        &self,
        target: impl Into<AddressBookEntryTarget>,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<bool> {
        let target = target.into();
        self.update_profile_with(|profile| {
            let current_network = ensure_on_current_network(profile, &target)?;

            let mut did_update = false;
            profile.networks.update_with(current_network, |network| {
                did_update = network.address_book.update_entry(
                    target.clone(),
                    name,
                    note.clone(),
                );
            });
            Ok(did_update)
        })
        .await
    }

    /// Deletes an address book entry on the current network.
    /// Returns false if no entry names `target`.
    pub async fn delete_address_book_entry(
        &self,
        target: impl Into<AddressBookEntryTarget>,
    ) -> Result<bool> {
        let target = target.into();
        self.update_profile_with(|profile| {
            let current_network = ensure_on_current_network(profile, &target)?;

            let mut did_delete = false;
            profile.networks.update_with(current_network, |network| {
                did_delete = network.address_book.remove(&target);
            });
            Ok(did_delete)
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SargonOS;

    #[actix_rt::test]
    async fn add_address_book_entry_adds_new_entry() {
        let os = SUT::fast_boot().await;
        let address = Address::Resource(ResourceAddress::sample_mainnet_xrd());
        os.with_timeout(|x| {
            x.add_address_book_entry(
                address,
                DisplayName::sample(),
                Some("Exchange".to_owned()),
            )
        })
        .await
        .unwrap();
        let stored = os.address_book_entry(address).unwrap();
        assert_eq!(stored.name, DisplayName::sample());
        assert_eq!(stored.note, Some("Exchange".to_owned()));
    }

    #[actix_rt::test]
    async fn account_address_book_on_current_network_filters_non_accounts() {
        let os = SUT::fast_boot().await;
        let account_address =
            Address::Account(AccountAddress::sample_mainnet_other());
        let resource_address =
            Address::Resource(ResourceAddress::sample_mainnet_xrd());

        os.with_timeout(|x| {
            x.add_address_book_entry(
                account_address,
                DisplayName::sample(),
                Some("Account".to_owned()),
            )
        })
        .await
        .unwrap();
        os.with_timeout(|x| {
            x.add_address_book_entry(
                resource_address,
                DisplayName::sample_other(),
                Some("Resource".to_owned()),
            )
        })
        .await
        .unwrap();

        let entries = os.account_address_book_on_current_network().unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].target, account_address.into());
    }

    #[actix_rt::test]
    async fn add_address_book_entry_duplicate_returns_alias_error() {
        let os = SUT::fast_boot().await;
        let address = Address::Resource(ResourceAddress::sample_mainnet_xrd());

        os.with_timeout(|x| {
            x.add_address_book_entry(
                address,
                DisplayName::sample(),
                Some("Exchange".to_owned()),
            )
        })
        .await
        .unwrap();

        let err = os
            .with_timeout(|x| {
                x.add_address_book_entry(
                    address,
                    DisplayName::sample_other(),
                    Some("Another".to_owned()),
                )
            })
            .await
            .unwrap_err();

        assert_eq!(
            err,
            CommonError::AddressBookEntryAlreadyExists {
                alias: DisplayName::sample().to_string(),
            }
        );
    }

    #[actix_rt::test]
    async fn update_address_book_entry_updates_and_bumps_updated_at() {
        let os = SUT::fast_boot().await;
        let address =
            Address::Identity(IdentityAddress::sample_mainnet_other());
        os.with_timeout(|x| {
            x.add_address_book_entry(
                address,
                DisplayName::sample(),
                Some("first".to_owned()),
            )
        })
        .await
        .unwrap();

        let before = os.address_book_entry(address).unwrap();
        actix_rt::time::sleep(std::time::Duration::from_millis(2)).await;
        let did_update = os
            .with_timeout(|x| {
                x.update_address_book_entry(
                    address,
                    DisplayName::sample_other(),
                    Some("second".to_owned()),
                )
            })
            .await
            .unwrap();
        let after = os.address_book_entry(address).unwrap();

        assert!(did_update);
        assert_eq!(after.created_at, before.created_at);
        assert!(after.updated_at >= before.updated_at);
        assert_eq!(after.name, DisplayName::sample_other());
        assert_eq!(after.note, Some("second".to_owned()));
    }

    #[actix_rt::test]
    async fn update_or_delete_missing_address_book_entry_returns_false() {
        let os = SUT::fast_boot().await;
        let address = Address::Locker(LockerAddress::sample_mainnet_other());

        let did_update = os
            .with_timeout(|x| {
                x.update_address_book_entry(
                    address,
                    DisplayName::sample(),
                    Some("x".to_owned()),
                )
            })
            .await
            .unwrap();
        let did_delete = os
            .with_timeout(|x| x.delete_address_book_entry(address))
            .await
            .unwrap();

        assert!(!did_update);
        assert!(!did_delete);
    }

    #[actix_rt::test]
    async fn address_book_cross_network_is_rejected() {
        let os = SUT::fast_boot().await;
        let wrong_network_address =
            Address::Validator(ValidatorAddress::sample_stokenet());

        let err = os
            .with_timeout(|x| {
                x.add_address_book_entry(
                    wrong_network_address,
                    DisplayName::sample(),
                    Some("x".to_owned()),
                )
            })
            .await
            .unwrap_err();

        assert!(matches!(err, CommonError::EntityOnWrongNetwork { .. }));
    }

    #[actix_rt::test]
    async fn non_fungible_is_named_and_not_listed_as_account() {
        let os = SUT::fast_boot().await;
        let key = NonFungibleGlobalId::sample();

        os.with_timeout(|x| {
            x.add_address_book_entry(key.clone(), DisplayName::sample(), None)
        })
        .await
        .unwrap();

        assert_eq!(
            os.address_book_entry(key.clone()).unwrap().name,
            DisplayName::sample()
        );
        assert!(os
            .account_address_book_on_current_network()
            .unwrap()
            .is_empty());

        let did_delete = os
            .with_timeout(|x| x.delete_address_book_entry(key.clone()))
            .await
            .unwrap();
        assert!(did_delete);
        assert!(os.address_book_entry(key).is_err());
    }
}
