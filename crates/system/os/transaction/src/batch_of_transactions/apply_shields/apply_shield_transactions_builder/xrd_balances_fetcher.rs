use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsXrdBalancesFetcher: Send + Sync {
    async fn get_xrd_balances(
        &self,
        network_id: NetworkID,
        manifests_with_entities_without_xrd_balances: Vec<
            ShieldApplicationInputWithoutXrdBalance,
        >,
    ) -> Result<Vec<ShieldApplicationInput>>;
}

pub struct ApplyShieldTransactionsXrdBalancesFetcherImpl {
    networking_driver: Arc<dyn NetworkingDriver>,
}

impl ApplyShieldTransactionsXrdBalancesFetcherImpl {
    pub fn new(networking_driver: Arc<dyn NetworkingDriver>) -> Self {
        Self { networking_driver }
    }

    /// Also fetched XRD balances of AccessControllers of securified **Personas**
    async fn batch_fetch_xrd_balances_of_accounts_or_access_controllers(
        &self,
        network_id: NetworkID,
        addresses: IndexSet<AddressOfPayerOfShieldApplication>,
    ) -> Result<XrdBalances> {
        assert!(addresses.iter().all(|a| a.network_id() == network_id));
        let gateway_client =
            GatewayClient::new(self.networking_driver.clone(), network_id);

        let balances = gateway_client
            .xrd_balances_of_vault_or_account(network_id, addresses)
            .await?;

        let balances = balances
            .into_iter()
            .map(|(k, v)| (k, v.unwrap_or(Decimal192::zero())))
            .collect::<IndexMap<_, _>>();

        Ok(XrdBalances(balances))
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsXrdBalancesFetcher
    for ApplyShieldTransactionsXrdBalancesFetcherImpl
{
    async fn get_xrd_balances(
        &self,
        network_id: NetworkID,
        manifests_with_entities_without_xrd_balances: Vec<
            ShieldApplicationInputWithoutXrdBalance,
        >,
    ) -> Result<Vec<ShieldApplicationInput>> {
        let addresses_to_query = manifests_with_entities_without_xrd_balances
            .iter()
            .flat_map(|i| i.addresses_to_fetch_xrd_balance_for())
            .collect::<IndexSet<AddressOfPayerOfShieldApplication>>();

        let mut balances = self
            .batch_fetch_xrd_balances_of_accounts_or_access_controllers(
                network_id,
                addresses_to_query,
            )
            .await?;

        manifests_with_entities_without_xrd_balances
            .into_iter()
            .map(|i| {
                ShieldApplicationInput::taking_xrd_balances(i, &mut balances)
            })
            .collect::<Result<Vec<ShieldApplicationInput>>>()
    }
}

impl ShieldApplicationInput {
    pub fn taking_xrd_balances(
        input: ShieldApplicationInputWithoutXrdBalance,
        balances: &mut XrdBalances,
    ) -> Result<Self> {
        match input {
            ShieldApplicationInputWithoutXrdBalance::Unsecurified(unsec) => {
                unsec.into_shield_application_input_taking_from_xrd_balances(
                    balances,
                )
            }
            ShieldApplicationInputWithoutXrdBalance::Securified(sec) => sec
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                ),
        }
    }
}

impl ApplicationInputForUnsecurifiedEntityWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        match self {
            Self::Account(unsec_acc) => unsec_acc
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                ),
            Self::Persona(unsec_pers) => unsec_pers
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                ),
        }
    }
}

pub struct XrdBalances(IndexMap<AddressOfPayerOfShieldApplication, Decimal>);
impl XrdBalances {
    pub fn take(
        &mut self,
        address: impl Into<AddressOfPayerOfShieldApplication>,
    ) -> Result<Decimal> {
        self.0
            .shift_remove(&address.into())
            .ok_or(CommonError::Unknown) // TODO: Special Error case
    }

    fn take_securified_payer(
        &mut self,
        account: SecurifiedAccount,
    ) -> Result<ApplicationInputPayingAccount> {
     todo!()
    }
    fn take_unsecurified_payer(
        &mut self,
        account: UnsecurifiedAccount,
    ) -> Result<ApplicationInputPayingAccount> {
     todo!()
    }

    pub fn take_payer(
        &mut self,
        account: Account,
    ) -> Result<ApplicationInputPayingAccount> {
      SecurifiedAccount::try_from(account).and_then(self.take)
    }

    pub fn maybe_take_payer(
        &mut self,
        account: Option<Account>,
    ) -> Result<Option<ApplicationInputPayingAccount>> {
        if let Some(account) = account {
            self.take_payer(account).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl ApplicationInputForUnsecurifiedAccountWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        /*
                pub reviewed_manifest: TransactionManifest,
        pub estimated_xrd_fee: Decimal,
        pub entity_input: UnsecurifiedAccount,
        pub maybe_paying_account: Option<Account>,
            */
        // UnsecurifiedAccount
        let xrd_balance_of_account =
            balances.take(self.entity_input.entity.address)?;

        Ok(ApplicationInputForUnsecurifiedAccount {
            entity_input: UnsecurifiedAccountEntityInput {
                unsecurified_entity: self.entity_input,
                xrd_balance_of_account,
            },
            maybe_paying_account: balances
                .maybe_take(self.maybe_paying_account).map(|payer_with_balance| ApplicationInputPayingAccount {

                }),
        })
    }
}

impl ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        todo!()
    }
}

impl ApplicationInputForSecurifiedEntityWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        match self {
            Self::Account(sec_acc) => sec_acc
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                ),
            Self::Persona(sec_pers) => sec_pers
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                ),
        }
    }
}

impl ApplicationInputForSecurifiedAccountWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        todo!()
    }
}

impl ApplicationInputForSecurifiedPersonaWithoutXrdBalance {
    pub fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
    ) -> Result<ShieldApplicationInput> {
        todo!()
    }
}
