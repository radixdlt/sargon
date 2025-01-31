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

        let balances = self
            .batch_fetch_xrd_balances_of_accounts_or_access_controllers(
                network_id,
                addresses_to_query,
            )
            .await?;

        manifests_with_entities_without_xrd_balances
                .into_iter()
                .map(|i| {
                    let entity_applying_shield_and_balance_res: Result<XrdBalanceOfEntity<EntityApplyingShield>> = match i.get_entity_applying_shield() {
                        EntityApplyingShield::Securified(e) => {
                            let vault_address = e.xrd_vault_address();
                            let balance = balances.get(&AddressOfVaultOrAccount::Vault(vault_address)).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(XrdBalanceOfEntity {
                                entity: EntityApplyingShield::securified(e),
                                balance
                            })
                        },
                        EntityApplyingShield::Unsecurified(e) => {
                            match &e.entity {
                                AccountOrPersona::AccountEntity(a) => {
                                    let balance = balances.get(&AddressOfVaultOrAccount::Account(a.address())).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(XrdBalanceOfEntity {
                                entity: EntityApplyingShield::unsecurified_account(UnsecurifiedAccount::with_unsecured_entity_control(a.clone(), e.unsecured_entity_control.clone())),
                                balance
                            })
                                }
                                AccountOrPersona::PersonaEntity(p) => {
                                    // Unsecurified Personas cannot have any XRD... 
                                    // thus we use Decimal192::zero(), which is a safe default
                                    // we can update the types involved in this function
                                    // to make this exeuction path impossible, alas,
                                    // they are already too complex, so seems no worth it.
                                    Ok(XrdBalanceOfEntity {
                                        entity: EntityApplyingShield::unsecurified_persona(UnsecurifiedPersona::with_unsecured_entity_control(p.clone(), e.unsecured_entity_control.clone())),
                                        balance: Decimal192::zero()
                                    })
                                }
                            }
                        },
                    };
                    let entity_applying_shield_and_balance = entity_applying_shield_and_balance_res?;
                    match i.get_payer() {
                        Some(payer) => {
                            let balance = balances.get(&AddressOfVaultOrAccount::Account(payer.address())).ok_or(CommonError::Unknown).cloned()?; // TODO better error
                            Ok(ShieldApplicationInput::new(XrdBalanceOfEntity::<Account> {
                                entity: payer,
                                balance
                            }, entity_applying_shield_and_balance, i.manifest, i.estimated_xrd_fee))
                        }
                        None => {
                            Ok(ShieldApplicationInput::new(None, entity_applying_shield_and_balance, i.manifest, i.estimated_xrd_fee))
                        }
                    }
                })
                .collect::<Result<Vec<ShieldApplicationInput>>>()
    }
}
