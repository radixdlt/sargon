use crate::prelude::*;

pub trait ProfileMarkEntityAsSecurified {
    fn mark_entity_as_securified(
        &mut self,
        access_controller_address: AccessControllerAddress,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona>;
}

impl ProfileMarkEntityAsSecurified for Profile {
    fn mark_entity_as_securified(
        &mut self,
        access_controller_address: AccessControllerAddress,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona> {
        let mut entity = self.entity_by_address(entity_address)?;

        let transaction_signing = entity
            .security_state()
            .as_unsecured()
            .ok_or(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
            .map(|security_state| security_state.transaction_signing.clone())?;

        let security_structure_of_factor_instances = entity
            .get_provisional()
            .and_then(|p| p.into_factor_instances_derived().ok())
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?;

        let secured_entity_control = SecuredEntityControl::new(
            transaction_signing,
            access_controller_address,
            security_structure_of_factor_instances,
        )?;

        entity.set_provisional(None);
        entity.set_security_state(EntitySecurityState::Securified {
            value: secured_entity_control,
        })?;

        Ok(entity)
    }
}
