use crate::prelude::*;

use super::XrdBalances;

/// Fetches XRD balances of AccessControllers of securified **Personas**
/// and **Accounts** and the XRD balances of the accounts applying the shield.
/// (Personas do not have an XRD balance).
///
/// And ornaments `Vec<ShieldApplicationInputWithoutXrdBalance>` with the
/// fetched XRD balances and returns `Vec<ShieldApplicationInput>` with
/// which we are ready to modify the manifests and add Lock fee and XRD
/// vault top up instructions.
#[async_trait::async_trait]
pub trait ApplyShieldTransactionsXrdBalancesFetcher: Send + Sync {
    /// Fetches XRD balances of AccessControllers of securified **Personas**
    /// and **Accounts** and the XRD balances of the accounts applying the shield.
    /// (Personas do not have an XRD balance).
    ///
    /// And ornaments the inputs with the fetched XRD balances.
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

    /// Fetches XRD balances of AccessControllers of securified **Personas**
    /// and **Accounts** and the XRD balances of the accounts applying the shield.
    /// (Personas do not have an XRD balance).
    async fn batch_fetch_xrd_balances_of_accounts_or_access_controllers(
        &self,
        network_id: NetworkID,
        addresses: IndexSet<AddressOfPayerOfShieldApplication>,
    ) -> Result<XrdBalances> {
        assert!(addresses.iter().all(|a| a.network_id() == network_id));
        let gateway_client =
            GatewayClient::new(self.networking_driver.clone(), network_id);

        let balances = gateway_client
            .xrd_balances_of_access_controller_or_account(network_id, addresses)
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
    /// Fetches XRD balances of AccessControllers of securified **Personas**
    /// and **Accounts** and the XRD balances of the accounts applying the shield.
    /// (Personas do not have an XRD balance).
    ///
    /// And ornaments the inputs with the fetched XRD balances.
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
    /// Ornaments the input with the XRD balances fetched from the `XrdBalances`.
    fn taking_xrd_balances(
        input: ShieldApplicationInputWithoutXrdBalance,
        balances: &mut XrdBalances,
    ) -> Result<Self> {
        let fee_tip_percentage = input.fee_tip_percentage();
        match input {
            ShieldApplicationInputWithoutXrdBalance::Unsecurified(unsec) => {
                unsec.into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                )
            }
            ShieldApplicationInputWithoutXrdBalance::Securified(sec) => sec
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                ),
        }
    }
}

impl ApplicationInputForUnsecurifiedEntityWithoutXrdBalance {
    /// Ornaments this input with the XRD balance of the unsecurified entity applying the shield.
    /// and the XRD balance of the paying account (if any).
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        match self {
            Self::Account(unsec_acc) => unsec_acc
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                ),
            Self::Persona(unsec_pers) => unsec_pers
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                ),
        }
    }
}

impl ApplicationInputForUnsecurifiedAccountWithoutXrdBalance {
    /// Ornaments this input with the XRD balance of the account applying the shield.
    /// and with the XRD balance of the paying account (if any).
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        let xrd_balance_of_account = balances
            .get_xrd_balance_of_paying_component(
                self.entity_input.entity.address,
            )?;

        let paying_account = balances.get_payer(self.paying_account)?;

        Ok(ApplicationInputForUnsecurifiedAccount::new(
            self.reviewed_manifest,
            self.estimated_xrd_fee,
            UnsecurifiedAccountEntityInput {
                unsecurified_entity: self.entity_input,
                xrd_balance_of_account,
            },
            paying_account,
            fee_tip_percentage,
        )
        .into())
    }
}

impl ApplicationInputForUnsecurifiedPersonaWithoutXrdBalance {
    /// Ornaments this input with the XRD balance of the paying account.
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        let paying_account = balances.get_payer(self.paying_account)?;

        Ok(ApplicationInputForUnsecurifiedPersona::new(
            self.reviewed_manifest,
            self.estimated_xrd_fee,
            self.entity_input,
            paying_account,
            fee_tip_percentage,
        )
        .into())
    }
}

impl ApplicationInputForSecurifiedEntityWithoutXrdBalance {
    /// Ornaments this input with the XRD balances of the AccessControllers of the
    /// securified entity. and with the XRD balance of the paying account (if any),
    /// and with the XRD balance of the entitu applying the shield if it an Account
    /// (Persona does not have an XRD balance).
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        match self {
            Self::Account(sec_acc) => sec_acc
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                ),
            Self::Persona(sec_pers) => sec_pers
                .into_shield_application_input_taking_from_xrd_balances(
                    balances,
                    fee_tip_percentage,
                ),
        }
    }
}

impl ApplicationInputForSecurifiedAccountWithoutXrdBalance {
    /// Ornaments this input with the XRD balances of the AccessControllers of the
    /// securified **Account**. and with the XRD balance of the paying account (if
    /// any), and with the XRD balance of the account applying the shield.
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        let xrd_balance_of_account = balances
            .get_xrd_balance_of_paying_component(
                self.entity_input.entity.address,
            )?;

        let paying_account = balances.get_payer(self.paying_account)?;

        let xrd_balance_of_access_controller = balances
            .get_xrd_balance_of_paying_component(
                self.entity_input
                    .securified_entity_control()
                    .access_controller_address(),
            )?;

        Ok(ApplicationInputForSecurifiedAccount::new(
            self.reviewed_manifest,
            self.estimated_xrd_fee,
            SecurifiedAccountEntityInput {
                securified_account: self.entity_input,
                xrd_balance_of_access_controller,
                xrd_balance_of_account,
            },
            paying_account,
            fee_tip_percentage,
        )
        .into())
    }
}

impl ApplicationInputForSecurifiedPersonaWithoutXrdBalance {
    /// Ornaments this input with the XRD balances of the AccessControllers of the
    /// securified **Persona**. and with the XRD balance of the paying account.
    fn into_shield_application_input_taking_from_xrd_balances(
        self,
        balances: &mut XrdBalances,
        fee_tip_percentage: Option<u16>,
    ) -> Result<ShieldApplicationInput> {
        let paying_account = balances.get_payer(self.paying_account)?;

        let xrd_balance_of_access_controller = balances
            .get_xrd_balance_of_paying_component(
                self.entity_input
                    .securified_entity_control()
                    .access_controller_address(),
            )?;

        Ok(ApplicationInputForSecurifiedPersona::new(
            self.reviewed_manifest,
            self.estimated_xrd_fee,
            SecurifiedPersonaEntityInput {
                securified_persona: self.entity_input,
                xrd_balance_of_access_controller,
            },
            paying_account,
            fee_tip_percentage,
        )
        .into())
    }
}
