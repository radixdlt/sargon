use crate::prelude::*;

/// A map of AddressOfPayerOfShieldApplication and their XRD balances.
pub struct XrdBalances(
    pub(super) IndexMap<AddressOfPayerOfShieldApplication, Decimal>,
);

impl XrdBalances {
    /// Takes the XRD balance of the entity applying the shield, i.e. consumes it
    /// and returns it. We can do this since each entity applying a shield should
    /// occur only once in balances map.
    ///
    /// # Errors
    /// Throws an error if no XRD balance of `account` is found.
    pub fn take_for_entity_applying_shield(
        &mut self,
        address: impl Into<AddressOfPayerOfShieldApplication>,
    ) -> Result<Decimal> {
        let address = address.into();
        self.0
            .get(&address)
            .ok_or(CommonError::NoXrdBalanceFetchedForEntityApplyingShieldOrItsVault {
                address: address.to_string(),
            })
            .cloned()
    }

    /// Reads the XRD balance of the payer of the shield application - without
    /// consuming it, since the same payer can be used for multiple shield applications.
    ///     
    /// # Errors
    /// Throws an error if no XRD balance of `account` is found.
    fn get_paying_component(
        &mut self,
        address_of_payer: impl Into<AddressOfPayerOfShieldApplication>,
    ) -> Result<Decimal> {
        let address_of_payer = address_of_payer.into();
        self.0
            .get(&address_of_payer)
            .ok_or(
                CommonError::NoXrdBalanceFetchedForPayerOfApplicationOfShield {
                    address_of_payer: address_of_payer.to_string(),
                },
            )
            .cloned()
    }

    /// Reads the XRD balance of the payer of the shield application - without
    /// consuming it, since the same payer can be used for multiple shield applications.
    ///   
    /// # Errors
    /// Throws an error if no XRD balance of `account` is found.
    fn get_securified_payer(
        &mut self,
        account: SecurifiedAccount,
    ) -> Result<ApplicationInputPayingAccount> {
        let account_address = account.entity.address;
        let sec = account.securified_entity_control();
        let xrd_vault_address = sec.xrd_vault_address();
        let access_controller_address = sec.access_controller_address();

        let xrd_balance_of_account =
            self.get_paying_component(account_address)?;
        let xrd_balance_of_access_controller =
            self.get_paying_component(xrd_vault_address)?;

        Ok(ApplicationInputPayingAccount::Securified(
            ApplicationInputPayingAccountSecurified {
                account,
                access_controller_address,
                xrd_vault_address,
                xrd_balance_of_access_controller,
                xrd_balance_of_account,
            },
        ))
    }

    /// Reads the XRD balance of the payer of the shield application - without
    /// consuming it, since the same payer can be used for multiple shield applications.    
    ///
    /// # Errors
    /// Throws an error if no XRD balance of `account` is found.
    fn get_unsecurified_payer(
        &mut self,
        account: UnsecurifiedAccount,
    ) -> Result<ApplicationInputPayingAccount> {
        let account_address = account.entity.address;
        let xrd_balance_of_account =
            self.get_paying_component(account_address)?;
        Ok(ApplicationInputPayingAccount::Unsecurified(
            ApplicationInputPayingAccountUnsecurified {
                account,
                xrd_balance_of_account,
            },
        ))
    }

    /// Reads the XRD balance of the payer of the shield application - without
    /// consuming it, since the same payer can be used for multiple shield applications.
    ///
    /// # Errors
    /// Throws an error if no XRD balance of `account` is found.
    pub fn get_payer(
        &mut self,
        account: Account,
    ) -> Result<ApplicationInputPayingAccount> {
        SecurifiedAccount::try_from(account.clone())
            .and_then(|sa| self.get_securified_payer(sa))
            .or(UnsecurifiedAccount::try_from(account)
                .and_then(|ua| self.get_unsecurified_payer(ua)))
    }

    /// Tries to read the XRD balance of the payer of the shield application - without
    /// consuming it, since the same payer can be used for multiple shield applications.
    ///
    /// Returns None if the account is None.
    pub fn maybe_get_payer(
        &mut self,
        account: Option<Account>,
    ) -> Result<Option<ApplicationInputPayingAccount>> {
        if let Some(account) = account {
            self.get_payer(account).map(Some)
        } else {
            Ok(None)
        }
    }
}
