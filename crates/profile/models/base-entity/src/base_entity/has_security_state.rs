use crate::prelude::*;

pub trait HasSecurityState: HasFactorInstances + IsSecurityStateAware {
    fn security_state(&self) -> EntitySecurityState;
    fn set_security_state_unchecked(&mut self, new_state: EntitySecurityState);

    // TODO: Should we check `provisional_securified_config` of `self` and/or
    // of `new_state`?
    fn set_security_state(
        &mut self,
        new_state: EntitySecurityState,
    ) -> Result<()> {
        match (&self.security_state(), &new_state) {
            (
                &EntitySecurityState::Securified { .. },
                &EntitySecurityState::Unsecured { .. },
            ) => {
                Err(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
            }
            (
                EntitySecurityState::Securified {
                    value: sec_existing,
                },
                EntitySecurityState::Securified { value: sec_new },
            ) => {
                if sec_new.access_controller_address()
                    != sec_existing.access_controller_address()
                {
                    Err(CommonError::SecurityStateAccessControllerAddressMismatch)
                } else {
                    self.set_security_state_unchecked(new_state);
                    Ok(())
                }
            }
            _ => {
                self.set_security_state_unchecked(new_state);
                Ok(())
            }
        }
    }

    fn try_get_secured_control(&self) -> Result<SecuredEntityControl> {
        self.security_state()
            .as_securified()
            .cloned()
            .ok_or(CommonError::SecurityStateNotSecurified)
    }

    fn try_get_unsecured_control(&self) -> Result<UnsecuredEntityControl> {
        self.security_state()
            .as_unsecured()
            .cloned()
            .ok_or(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
    }
}
