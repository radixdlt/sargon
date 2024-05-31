use crate::prelude::*;
use std::future::Future;

impl Profile {
    /// Creates a new non securified account **WITHOUT** adding it to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" index for this FactorSource as derivation path.
    ///
    /// If you want to add it to Profile, call `add_account(account)`
    ///
    /// Returns a tuple `(FactorSourceID, Account)` where FactorSourceID is the ID
    /// of the FactorSource used to create the account.
    pub async fn create_unsaved_account<F, Fut>(
        &self,
        network_id: NetworkID,
        name: DisplayName,
        load_private_device_factor_source: F,
    ) -> Result<(FactorSourceID, Account)>
    where
        F: FnOnce(DeviceFactorSource) -> Fut,
        Fut: Future<
            Output = Result<PrivateHierarchicalDeterministicFactorSource>,
        >,
    {
        let (factor_source_id, accounts) = self
            .create_unsaved_accounts(
                network_id,
                1,
                |_| name.clone(),
                load_private_device_factor_source,
            )
            .await?;

        let account = accounts
            .into_iter()
            .last()
            .expect("Should have created one account");

        Ok((factor_source_id, account))
    }

    /// Creates many new non securified accounts **WITHOUT** adding them to Profile, using the *main* "Babylon"
    /// `DeviceFactorSource` and the "next" indices for this FactorSource as derivation paths.
    ///
    /// If you want to add the accounts to Profile, call `add_accounts(accounts)`
    ///
    /// Returns a tuple `(FactorSourceID, Accounts)` where FactorSourceID is the ID
    /// of the FactorSource used to create the accounts.
    pub async fn create_unsaved_accounts<F, Fut>(
        &self,
        network_id: NetworkID,
        count: u16,
        get_name: impl Fn(u32) -> DisplayName, // name of account at index
        load_private_device_factor_source: F,
    ) -> Result<(FactorSourceID, Accounts)>
    where
        F: FnOnce(DeviceFactorSource) -> Fut,
        Fut: Future<
            Output = Result<PrivateHierarchicalDeterministicFactorSource>,
        >,
    {
        let index = self
            .next_derivation_index_for_entity(EntityKind::Account, network_id);

        assert!((index as i64) - (count as i64) < (u32::MAX as i64)); // unlikely edge case

        let bdfs = self.bdfs();
        let count = count as u32;

        let number_of_accounts_on_network = self
            .networks
            .get_id(network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);

        let indices = index..index + count;

        let factor_instances = load_private_device_factor_source(bdfs.clone())
            .await
            .map(|p| {
                assert_eq!(p.factor_source, bdfs);
                p.derive_entity_creation_factor_instances(network_id, indices)
            })?;

        let accounts = factor_instances
            .into_iter()
            .map(|f| {
                let idx = f.index();
                let name = get_name(idx);
                let appearance_id =
                    AppearanceID::from_number_of_accounts_on_network(
                        (idx as usize) + number_of_accounts_on_network,
                    );

                Account::new(f, name, appearance_id)
            })
            .collect::<Accounts>();

        Ok((bdfs.factor_source_id().clone(), accounts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_unsaved_accounts() {
        let sut = Profile::from_device_factor_source(
            PrivateHierarchicalDeterministicFactorSource::sample()
                .factor_source,
            DeviceInfo::sample(),
        );

        let (_, accounts) = sut
            .create_unsaved_accounts(
                NetworkID::Mainnet,
                3,
                |i| {
                    DisplayName::new(if i == 0 {
                        "Alice"
                    } else if i == 1 {
                        "Bob"
                    } else {
                        "Carol"
                    })
                    .unwrap()
                },
                async move |_| {
                    Ok(PrivateHierarchicalDeterministicFactorSource::sample())
                },
            )
            .await
            .unwrap();

        pretty_assertions::assert_eq!(
            accounts,
            Accounts::from_iter([
                Account::sample_mainnet_alice(),
                Account::sample_mainnet_bob(),
                Account::sample_mainnet_carol()
            ])
        )
    }
}
