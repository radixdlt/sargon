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
        let bdfs = self.bdfs();
        todo!()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_create_unsaved_accounts() {
        let sut = Profile::from_device_factor_source(
            PrivateHierarchicalDeterministicFactorSource::sample()
                .factor_source,
            HostId::sample(),
            HostInfo::sample(),
            None::<Accounts>,
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

*/
