use crate::{
    Account, AccountAddress, AppearanceID, CommonError, DisplayName, EntityKind, Mnemonic,
    MnemonicWithPassphrase, NetworkID, PrivateHierarchicalDeterministicFactorSource,
    SecureStorageKey, Wallet,
};

//========
// SET - Account
//========
#[uniffi::export]
impl Wallet {
    /// Creates a new non securified account using the `main` "Babylon" `DeviceFactorSource` and the "next" index for this FactorSource
    /// as derivation path.
    pub fn create_new_account(
        &self,
        network_id: NetworkID,
        name: DisplayName,
    ) -> Result<Account, CommonError> {
        let profile = &self.profile();
        let bdfs = profile.bdfs();
        let index = profile.next_derivation_index_for_entity(EntityKind::Accounts, network_id);
        self.read_secure_storage
            .get(SecureStorageKey::DeviceFactorSourceMnemonic {
                factor_source_id: bdfs.clone().id,
            })
            .and_then(|o| o.ok_or(CommonError::SecureStorageReadError))
            .and_then(|p| Mnemonic::from_phrase(p.as_str()).map_err(|e| CommonError::HDPath(e)))
            .map(|m| {
                PrivateHierarchicalDeterministicFactorSource::new(
                    MnemonicWithPassphrase::new(m.clone()),
                    bdfs.clone(),
                )
            })
            .map(|p| p.derive_account_creation_factor_instance(network_id, index))
            .map(|fi| Account::new(fi, name, AppearanceID::new(0).unwrap()))
    }

    /// Updates the display name of account with the provided address, throws an error if the account is unknown to the wallet.
    pub fn change_name_of_account(
        &self,
        address: AccountAddress,
        to: DisplayName,
    ) -> Result<Account, CommonError> {
        self.write(|mut p| p.update_account(&address, |a| a.display_name = to.to_owned()))
            .ok_or_else(|| CommonError::UnknownAccount)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AccountAddress, CommonError, DisplayName, HasPlaceholder, NotYetSetSecureStorage, Profile,
        Wallet,
    };

    #[test]
    fn change_display_name_of_accounts() {
        let profile = Profile::placeholder();
        let wallet = Wallet::new(profile.clone(), NotYetSetSecureStorage::new());
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
