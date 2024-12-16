use crate::prelude::*;

pub trait HasSecurityState: HasFactorInstances + IsSecurityStateAware {
    fn security_state(&self) -> EntitySecurityState;

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

impl<T: HasSecurityState> HasFactorInstances for T {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_state().unique_tx_signing_factor_instances()
    }
}
