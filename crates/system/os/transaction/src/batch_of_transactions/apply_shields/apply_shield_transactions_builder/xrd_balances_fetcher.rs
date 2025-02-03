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
    ) -> Result<IndexMap<AddressOfPayerOfShieldApplication, Decimal>> {
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

        Ok(balances)
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
        balances: &mut IndexMap<AddressOfPayerOfShieldApplication, Decimal>,
    ) -> Result<Self> {
        todo!()
    }
}
