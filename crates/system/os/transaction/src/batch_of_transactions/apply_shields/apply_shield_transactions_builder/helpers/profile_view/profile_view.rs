use crate::prelude::*;

/// A "view" into Profile which provides methods for looking up entities by
/// their addresses.
#[async_trait::async_trait]
pub trait ApplyShieldTransactionsProfileView: Send + Sync {
    fn lookup_entities_for_manifests(
        &self,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
    ) -> Result<Vec<ShieldApplicationInputWithoutXrdBalance>>;
}

/// A "view" into Profile which provides methods for looking up entities by
/// their addresses.
pub struct ApplyShieldTransactionsProfileViewImpl {
    profile: Profile,
}

impl ApplyShieldTransactionsProfileViewImpl {
    pub fn new(profile: Profile) -> Self {
        Self { profile }
    }
}

impl ApplyShieldTransactionsProfileViewImpl {
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
            .any(|i| {
                payer_addresses.contains(&i.address_of_entity_applying_shield())
            })
        {
            return Err(
                CommonError::PayerCannotBeInBatchOfEntitiesApplyingShield,
            );
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
impl ApplyShieldTransactionsProfileView
    for ApplyShieldTransactionsProfileViewImpl
{
    fn lookup_entities_for_manifests(
        &self,
        manifest_and_payer_tuples: Vec<ManifestWithPayerByAddress>, // TODO: Want IndexSet but not Hash
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

                let maybe_entity_paying: Option<Account> = {
                    if let Some(address_of_paying_account) = manifest_with_payer_by_address.payer {
                        self.account_by_address(address_of_paying_account).map(Some)
                    } else {
                        Ok(None)
                    }
                }?;

                ShieldApplicationInputWithoutXrdBalance::new(
                    estimated_xrd_fee,
                    manifest,
                    entity_applying_shield,
                    maybe_entity_paying
                )
            })
            .collect::<Result<Vec<ShieldApplicationInputWithoutXrdBalance>>>()?;

        // Assert that payer if specified is not part of the batch of entities applying shield
        self.assert_that_payer_is_not_in_batch_of_entities_applying_shield(
            &manifests_with_entities_without_xrd_balances,
        )?;

        Ok(manifests_with_entities_without_xrd_balances)
    }
}

impl ShieldApplicationInputWithoutXrdBalance {
    pub fn new(
        estimated_xrd_fee: Decimal,
        manifest: TransactionManifest,
        entity_applying_shield: EntityApplyingShield,
        maybe_paying_account: Option<Account>,
    ) -> Result<Self> {
        let self_ = match entity_applying_shield {
            EntityApplyingShield::Unsecurified(unsec) => {
                let entity = match &unsec.entity {
                    AccountOrPersona::AccountEntity(a) => {
                        let a = ApplicationInputForUnsecurifiedAccountWithoutXrdBalance {
                            reviewed_manifest: manifest,
                            estimated_xrd_fee,
                            entity_input: UnsecurifiedAccount::with_unsecured_entity_control(a.clone(), unsec.unsecured_entity_control),
                            maybe_paying_account,
                        };
                        ApplicationInputForUnsecurifiedEntityWithoutXrdBalance::from(a)
                    }
                    AccountOrPersona::PersonaEntity(p) => {
                        let paying_account =
                            maybe_paying_account.ok_or(CommonError::UnsecurifiedPersonasRequireAnAccountFeePayerButNoneWasProvided {
                                identity_address: p.address.to_string(),
                            })?; // TODO Add new error type
                        let p = ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance {
                            reviewed_manifest: manifest,
                            estimated_xrd_fee,
                            entity_input: UnsecurifiedPersona::with_unsecured_entity_control(p.clone(), unsec.unsecured_entity_control),
                            paying_account,
                        };
                        ApplicationInputForUnsecurifiedEntityWithoutXrdBalance::from(p)
                    }
                };
                Self::from(entity)
            }
            EntityApplyingShield::Securified(sec) => {
                let entity = match &sec.entity {
                    AccountOrPersona::AccountEntity(a) => {
                        let a = ApplicationInputForSecurifiedAccountWithoutXrdBalance {
                            reviewed_manifest: manifest,
                            estimated_xrd_fee,
                            entity_input: SecurifiedAccount::with_securified_entity_control(a.clone(), sec.securified_entity_control()),
                            maybe_paying_account,
                        };
                        ApplicationInputForSecurifiedEntityWithoutXrdBalance::from(a)
                    }
                    AccountOrPersona::PersonaEntity(p) => {
                        let p = ApplicationInputForSecurifiedPersonaWithoutXrdBalance {
                            reviewed_manifest: manifest,
                            estimated_xrd_fee,
                            entity_input: SecurifiedPersona::with_securified_entity_control(p.clone(), sec.securified_entity_control()),
                            maybe_paying_account,
                        };
                        ApplicationInputForSecurifiedEntityWithoutXrdBalance::from(p)
                    }
                };
                Self::from(entity)
            }
        };
        Ok(self_)
    }
}

impl From<ApplicationInputForUnsecurifiedAccountWithoutXrdBalance>
    for ApplicationInputForUnsecurifiedEntityWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForUnsecurifiedAccountWithoutXrdBalance,
    ) -> Self {
        Self::Account(value)
    }
}

impl From<ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance>
    for ApplicationInputForUnsecurifiedEntityWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance,
    ) -> Self {
        Self::Persona(value)
    }
}

impl From<ApplicationInputForSecurifiedAccountWithoutXrdBalance>
    for ApplicationInputForSecurifiedEntityWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForSecurifiedAccountWithoutXrdBalance,
    ) -> Self {
        Self::Account(value)
    }
}

impl From<ApplicationInputForSecurifiedPersonaWithoutXrdBalance>
    for ApplicationInputForSecurifiedEntityWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForSecurifiedPersonaWithoutXrdBalance,
    ) -> Self {
        Self::Persona(value)
    }
}

impl From<ApplicationInputForUnsecurifiedEntityWithoutXrdBalance>
    for ShieldApplicationInputWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForUnsecurifiedEntityWithoutXrdBalance,
    ) -> Self {
        Self::Unsecurified(value)
    }
}

impl From<ApplicationInputForSecurifiedEntityWithoutXrdBalance>
    for ShieldApplicationInputWithoutXrdBalance
{
    fn from(
        value: ApplicationInputForSecurifiedEntityWithoutXrdBalance,
    ) -> Self {
        Self::Securified(value)
    }
}
