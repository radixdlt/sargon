use crate::prelude::*;

impl Wallet {
    pub fn load_private_device_factor_source(
        &self,
        device_factor_source: DeviceFactorSource,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        self.wallet_client_storage
            .load_mnemonic_with_passphrase(&device_factor_source.id)
            .map(|mwp| PrivateHierarchicalDeterministicFactorSource::new(mwp, device_factor_source))
    }
    pub fn load_private_device_factor_source_by_id(
        &self,
        id: &FactorSourceIDFromHash,
    ) -> Result<PrivateHierarchicalDeterministicFactorSource> {
        let device_factor_source = self.profile().device_factor_source_by_id(id)?;
        self.load_private_device_factor_source(device_factor_source)
    }
}

//========
// SET - Account
//========
#[uniffi::export]
impl Wallet {
    /// Creates a new non securified account using the `main` "Babylon" `DeviceFactorSource` and the "next" index for this FactorSource
    /// as derivation path.
    pub fn create_new_account(&self, network_id: NetworkID, name: DisplayName) -> Result<Account> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();
        let index = profile.next_derivation_index_for_entity(EntityKind::Accounts, network_id);
        let number_of_accounts_on_network = profile
            .networks
            .get(&network_id)
            .map(|n| n.accounts.len())
            .unwrap_or(0);
        let appearance_id =
            AppearanceID::from_number_of_accounts_on_network(number_of_accounts_on_network);
        self.load_private_device_factor_source(bdfs)
            .map(|p| p.derive_account_creation_factor_instance(network_id, index))
            .map(|fi| Account::new(fi, name, appearance_id))
    }

    /// Updates the display name of account with the provided address, throws an error if the account is unknown to the wallet.
    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account> {
        self.write(|mut p| p.update_account(&address, |a| a.display_name = to.to_owned()))
            .ok_or_else(|| CommonError::UnknownAccount)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccountAddress, CommonError, DisplayName, HasPlaceholder, MockSecureStorage, Profile,
        Wallet,
    };

    #[test]
    fn change_display_name_of_accounts() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), MockSecureStorage::new());
        let account = wallet.read(|p| p.networks[0].accounts[0].clone());
        assert_eq!(account.display_name.value, "Alice");
        assert!(wallet
            .change_name_of_account(account.address, DisplayName::new("Stella").unwrap())
            .is_ok());
        wallet.read(|p| assert_eq!(p.networks[0].accounts[0].display_name.value, "Stella"));

        assert_eq!(
            wallet.change_name_of_account(
                AccountAddress::placeholder_other(),
                DisplayName::new("not used").unwrap()
            ),
            Err(CommonError::UnknownAccount)
        );
    }
}
