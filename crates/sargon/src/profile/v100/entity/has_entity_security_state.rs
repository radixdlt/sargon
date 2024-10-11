use crate::prelude::*;

pub trait HasEntitySecurityState {
    fn security_state(&self) -> EntitySecurityState;

    fn virtual_hierarchical_deterministic_factor_instances(
        &self,
        filter_key_kind: impl Into<Option<CAP26KeyKind>>,
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
        let Some(key_kind) = filter_key_kind.into() else {
            return factor_instances;
        };
        factor_instances
            .into_iter()
            .filter(|fi| fi.key_kind() == Some(key_kind))
            .collect()
    }
}
