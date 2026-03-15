use crate::prelude::*;

impl SargonOS {
    /// Returns all address book entries on the current network.
    pub fn address_book_on_current_network(&self) -> Result<AddressBook> {
        self.profile_state_holder.address_book_on_current_network()
    }

    /// Looks up an address book entry by address on the current network.
    pub fn address_book_entry_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<AddressBookEntry> {
        self.profile_state_holder
            .address_book_entry_by_address(address)
    }
}

impl SargonOS {
    /// Adds a new address book entry on the current network.
    /// Returns false if an entry with this address already exists.
    pub async fn add_address_book_entry(
        &self,
        address: AccountAddress,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<bool> {
        self.update_profile_with(|profile| {
            let current_network = profile.current_network_id();
            if address.network_id() != current_network {
                return Err(CommonError::EntityOnWrongNetwork {
                    entity_kind: "AddressBookEntry".to_owned(),
                    wrong_network: address.network_id().to_string(),
                    expected_network: current_network.to_string(),
                });
            }

            let entry = AddressBookEntry::new(address, name, note.clone());
            let mut did_add = false;
            profile.networks.update_with(current_network, |network| {
                did_add = network.address_book.add_entry(entry.clone());
            });
            Ok(did_add)
        })
        .await
    }

    /// Updates an existing address book entry on the current network.
    /// Returns false if no entry exists for the address.
    pub async fn update_address_book_entry(
        &self,
        address: AccountAddress,
        name: DisplayName,
        note: Option<String>,
    ) -> Result<bool> {
        self.update_profile_with(|profile| {
            let current_network = profile.current_network_id();
            if address.network_id() != current_network {
                return Err(CommonError::EntityOnWrongNetwork {
                    entity_kind: "AddressBookEntry".to_owned(),
                    wrong_network: address.network_id().to_string(),
                    expected_network: current_network.to_string(),
                });
            }

            let mut did_update = false;
            profile.networks.update_with(current_network, |network| {
                did_update = network.address_book.update_entry(
                    address,
                    name,
                    note.clone(),
                );
            });
            Ok(did_update)
        })
        .await
    }

    /// Deletes an address book entry on the current network.
    /// Returns false if no entry exists for the address.
    pub async fn delete_address_book_entry(
        &self,
        address: AccountAddress,
    ) -> Result<bool> {
        self.update_profile_with(|profile| {
            let current_network = profile.current_network_id();
            if address.network_id() != current_network {
                return Err(CommonError::EntityOnWrongNetwork {
                    entity_kind: "AddressBookEntry".to_owned(),
                    wrong_network: address.network_id().to_string(),
                    expected_network: current_network.to_string(),
                });
            }

            let mut did_delete = false;
            profile.networks.update_with(current_network, |network| {
                did_delete = network.address_book.remove_by_address(&address);
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
        let address = AccountAddress::sample_mainnet_other();
        let did_add = os
            .with_timeout(|x| {
                x.add_address_book_entry(
                    address,
                    DisplayName::sample(),
                    Some("Exchange".to_owned()),
                )
            })
            .await
            .unwrap();

        assert!(did_add);
        let stored = os.address_book_entry_by_address(address).unwrap();
        assert_eq!(stored.name, DisplayName::sample());
        assert_eq!(stored.note, Some("Exchange".to_owned()));
    }

    #[actix_rt::test]
    async fn add_address_book_entry_duplicate_returns_false() {
        let os = SUT::fast_boot().await;
        let address = AccountAddress::sample_mainnet_other();

        let _ = os
            .with_timeout(|x| {
                x.add_address_book_entry(
                    address,
                    DisplayName::sample(),
                    Some("Exchange".to_owned()),
                )
            })
            .await
            .unwrap();

        let did_add = os
            .with_timeout(|x| {
                x.add_address_book_entry(
                    address,
                    DisplayName::sample_other(),
                    Some("Another".to_owned()),
                )
            })
            .await
            .unwrap();

        assert!(!did_add);
    }

    #[actix_rt::test]
    async fn update_address_book_entry_updates_and_bumps_updated_at() {
        let os = SUT::fast_boot().await;
        let address = AccountAddress::sample_mainnet_other();
        os.with_timeout(|x| {
            x.add_address_book_entry(
                address,
                DisplayName::sample(),
                Some("first".to_owned()),
            )
        })
        .await
        .unwrap();

        let before = os.address_book_entry_by_address(address).unwrap();
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
        let after = os.address_book_entry_by_address(address).unwrap();

        assert!(did_update);
        assert_eq!(after.created_at, before.created_at);
        assert!(after.updated_at >= before.updated_at);
        assert_eq!(after.name, DisplayName::sample_other());
        assert_eq!(after.note, Some("second".to_owned()));
    }

    #[actix_rt::test]
    async fn update_or_delete_missing_address_book_entry_returns_false() {
        let os = SUT::fast_boot().await;
        let address = AccountAddress::sample_mainnet_other();

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
        let wrong_network_address = AccountAddress::sample_stokenet();

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
}
