use crate::prelude::*;

#[async_trait::async_trait]
pub trait ApplyShieldTransactionsProfileLens: Send + Sync {
    fn lookup_entities_for_manifests(
        &self,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<Vec<ShieldApplicationInputWithoutXrdBalance>>;
}

pub struct ApplyShieldTransactionsProfileLensImpl {
    profile: Profile,
}
impl ApplyShieldTransactionsProfileLensImpl {
    pub fn new(profile: Profile) -> Self {
        todo!()
    }
}

impl ApplyShieldTransactionsProfileLensImpl {
    /// Looks up the account by account address, returns Err if the account is
    /// unknown, will return a hidden account if queried for.
    pub fn account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<Account> {
        self.profile.account_by_address(address)
    }

    fn get_securified_entity_by_access_controller(
        &self,
        address: AccessControllerAddress,
    ) -> Result<AnySecurifiedEntity> {
        self.profile
            .get_securified_entity_by_access_controller_address(address)
    }

    fn get_unsecurified_account_by_address(
        &self,
        address: AccountAddress,
    ) -> Result<UnsecurifiedAccount> {
        self.profile
            .unsecurified_accounts_on_network(address.network_id())
            .iter()
            .find(|a| a.entity.address == address)
            .ok_or(CommonError::UnknownAccount)
    }

    fn get_unsecurified_persona_by_address(
        &self,
        address: IdentityAddress,
    ) -> Result<UnsecurifiedPersona> {
        self.profile
            .unsecurified_personas_on_network(address.network_id())
            .iter()
            .find(|a| a.entity.address == address)
            .ok_or(CommonError::UnknownPersona)
    }

    fn assert_that_payer_is_not_in_batch_of_entities_applying_shield(
        &self,
        manifests_with_entities_without_xrd_balances: impl AsRef<
            [ShieldApplicationInputWithoutXrdBalance],
        >,
    ) -> Result<()> {
        let payer_addresses = manifests_with_entities_without_xrd_balances
            .as_ref()
            .iter()
            .filter_map(|i| i.get_payer())
            .map(|a| a.address())
            .map(AddressOfAccountOrPersona::from)
            .collect::<IndexSet<_>>();

        if manifests_with_entities_without_xrd_balances
            .as_ref()
            .iter()
            .any(|i| payer_addresses.contains(&i.address_erased()))
        {
            return Err(CommonError::Unknown); // CommonError::PayerCannotBeInBatchOfEntitiesApplyingShield
        }

        Ok(())
    }

    fn get_entity_applying_shield(
        &self,
        address: EntityApplyingShieldAddress,
    ) -> Result<EntityApplyingShield> {
        match address {
            EntityApplyingShieldAddress::AccessController(ac) => self
                .get_securified_entity_by_access_controller(ac)
                .map(EntityApplyingShield::securified),
            EntityApplyingShieldAddress::Account(a) => self
                .get_unsecurified_account_by_address(a)
                .map(EntityApplyingShield::unsecurified_account),
            EntityApplyingShieldAddress::Identity(i) => self
                .get_unsecurified_persona_by_address(i)
                .map(EntityApplyingShield::unsecurified_persona),
        }
    }
}

#[async_trait::async_trait]
impl ApplyShieldTransactionsProfileLens
    for ApplyShieldTransactionsProfileLensImpl
{
    fn lookup_entities_for_manifests(
        &self,
        manifest_and_payer_tuples: IndexSet<ManifestWithPayerByAddress>,
    ) -> Result<Vec<ShieldApplicationInputWithoutXrdBalance>> {
        let manifests_with_entities_without_xrd_balances = manifest_and_payer_tuples
            .into_iter()
            .map(|manifest_with_payer_by_address| {
                let manifest = manifest_with_payer_by_address.manifest;
                let estimated_xrd_fee =
                    manifest_with_payer_by_address.estimated_xrd_fee;
                let address_of_ac_or_entity_applying_shield =
                    extract_address_of_entity_updating_shield(&manifest)?;

                let entity_applying_shield = self.get_entity_applying_shield(
                    address_of_ac_or_entity_applying_shield,
                )?;

                if let Some(payer_address) =
                    manifest_with_payer_by_address.payer
                {
                    let payer = self.account_by_address(payer_address)?;
                    Ok(ShieldApplicationInputWithoutXrdBalance::new(
                        payer,
                        entity_applying_shield,
                        manifest,
                        estimated_xrd_fee,
                    ))
                } else {
                    Ok(ShieldApplicationInputWithoutXrdBalance::new(
                        None,
                        entity_applying_shield,
                        manifest,
                        estimated_xrd_fee,
                    ))
                }
            })
            .collect::<Result<Vec<ShieldApplicationInputWithoutXrdBalance>>>()?;

        // Assert that payer if specified is not part of the batch of entities applying shield
        self.assert_that_payer_is_not_in_batch_of_entities_applying_shield(
            &manifests_with_entities_without_xrd_balances,
        )?;

        Ok(manifests_with_entities_without_xrd_balances)
    }
}
