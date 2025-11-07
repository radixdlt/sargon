use crate::prelude::*;

pub trait ProfileUpdateEntitySecurifiedState {
    fn commit_provisional_security_state(
        &mut self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona>;

    fn remove_provisional_security_state(
        &mut self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()>;
}

impl ProfileUpdateEntitySecurifiedState for Profile {
    fn commit_provisional_security_state(
        &mut self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<AccountOrPersona> {
        let mut entity = self.entity_by_address(entity_address)?;
        let mut secured_entity_control = entity
            .security_state()
            .as_securified()
            .ok_or(CommonError::SecurityStateNotSecurified)?
            .clone();

        let provisional_config = secured_entity_control
            .get_provisional()
            .ok_or(CommonError::EntityHasNoProvisionalSecurityConfigSet)?;

        let security_structure_of_factor_instances = provisional_config
        .as_factor_instances_derived()
        .ok_or(CommonError::ProvisionalConfigInWrongStateExpectedInstancesDerived)?;

        secured_entity_control.set_provisional(None);
        secured_entity_control.security_structure =
            security_structure_of_factor_instances.clone();

        entity.set_security_state(EntitySecurityState::Securified {
            value: secured_entity_control,
        })?;

        self.update_entities_erased(IdentifiedVecOf::just(entity.clone()))?;

        Ok(entity)
    }

    fn remove_provisional_security_state(
        &mut self,
        entity_address: AddressOfAccountOrPersona,
    ) -> Result<()> {
        let mut entity = self.entity_by_address(entity_address)?;
        let mut secured_entity_control = entity
            .security_state()
            .as_securified()
            .ok_or(CommonError::SecurityStateNotSecurified)?
            .clone();
        secured_entity_control.set_provisional(None);
        entity.set_security_state(EntitySecurityState::Securified {
            value: secured_entity_control,
        })?;
        self.update_entities_erased(IdentifiedVecOf::just(entity.clone()))?;
        Ok(())
    }
}
