use crate::prelude::*;

pub trait HasEntitySecurityState {
    fn security_state(&self) -> EntitySecurityState;

    fn virtual_hierarchical_deterministic_factor_instances(
        &self,
    ) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        let mut factor_instances: IndexSet<
            HierarchicalDeterministicFactorInstance,
        > = IndexSet::new();
        match self.security_state() {
            EntitySecurityState::Unsecured { value } => {
                factor_instances.insert(value.transaction_signing);
                if let Some(authentication_signing) =
                    value.authentication_signing
                {
                    factor_instances.insert(authentication_signing);
                }
            }
        }
        factor_instances
    }
}
