use crate::prelude::*;

pub trait FromHDFactorInstance: EntityAddress {
    fn from_hd_factor_instance_virtual_entity_creation<
        E: IsEntityPath + Clone,
    >(
        hd_factor_instance_virtual_entity_creation: HDFactorInstanceTransactionSigning<E>,
    ) -> Self {
        let network_id =
            hd_factor_instance_virtual_entity_creation.path.network_id();

        Self::from_public_key(
            hd_factor_instance_virtual_entity_creation
                .public_key()
                .public_key,
            network_id,
        )
    }
}
impl FromHDFactorInstance for AccountAddress {}
impl FromHDFactorInstance for IdentityAddress {}

pub trait FromAppearanceID {
    fn new(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self;
}

impl FromAppearanceID for Account {
    fn new(
        account_creating_factor_instance: HDFactorInstanceAccountCreation,
        display_name: DisplayName,
        appearance_id: AppearanceID,
    ) -> Self {
        let address =
            AccountAddress::from_hd_factor_instance_virtual_entity_creation(
                account_creating_factor_instance.clone(),
            );
        Self {
            network_id: account_creating_factor_instance.network_id(),
            address,
            display_name,
            security_state:
                UnsecuredEntityControl::with_entity_creating_factor_instance(
                    account_creating_factor_instance,
                )
                .into(),
            appearance_id,
            flags: EntityFlags::default(),
            on_ledger_settings: OnLedgerSettings::default(),
        }
    }
}
pub trait FromPersonaData {
    fn new(
        persona_creating_factor_instance: HDFactorInstanceIdentityCreation,
        display_name: DisplayName,
        persona_data: impl Into<Option<PersonaData>>,
    ) -> Self;
}
impl FromPersonaData for Persona {
    /// Creates a new `Persona`, if `persona_data` is `None`, an empty object will be created.
    fn new(
        persona_creating_factor_instance: HDFactorInstanceIdentityCreation,
        display_name: DisplayName,
        persona_data: impl Into<Option<PersonaData>>,
    ) -> Self {
        let address =
            IdentityAddress::from_hd_factor_instance_virtual_entity_creation(
                persona_creating_factor_instance.clone(),
            );
        Self {
            network_id: persona_creating_factor_instance.network_id(),
            address,
            display_name,
            security_state:
                UnsecuredEntityControl::with_entity_creating_factor_instance(
                    persona_creating_factor_instance,
                )
                .into(),
            flags: EntityFlags::default(),
            persona_data: persona_data.into().unwrap_or_default(),
        }
    }
}
