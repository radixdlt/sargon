use crate::prelude::*;

pub trait EntityUnsecuredControllingFactorInstance {
    fn unsecured_controlling_factor_instance(
        &self,
    ) -> Option<HierarchicalDeterministicFactorInstance>;
}

impl EntityUnsecuredControllingFactorInstance for Account {
    fn unsecured_controlling_factor_instance(
        &self,
    ) -> Option<HierarchicalDeterministicFactorInstance> {
        let unsecured_entity_control = self.security_state.as_unsecured()?;

        Some(unsecured_entity_control.transaction_signing.clone())
    }
}

impl EntityUnsecuredControllingFactorInstance for Persona {
    fn unsecured_controlling_factor_instance(
        &self,
    ) -> Option<HierarchicalDeterministicFactorInstance> {
        let unsecured_entity_control = self.security_state.as_unsecured()?;

        Some(unsecured_entity_control.transaction_signing.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsecured_account_returns_instance() {
        let sut = Account::sample();

        assert!(sut.unsecured_controlling_factor_instance().is_some());
    }

    #[test]
    fn test_secured_account_returns_none() {
        let sut = Account::sample_securified_mainnet(
            "Grace",
            6,
            HierarchicalDeterministicFactorInstance::sample_fia10(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Account,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        assert!(sut.unsecured_controlling_factor_instance().is_none());
    }

    #[test]
    fn test_unsecured_persona_returns_instance() {
        let sut = Persona::sample();

        assert!(sut.unsecured_controlling_factor_instance().is_some());
    }

    #[test]
    fn test_secured_persona_returns_none() {
        let sut = Persona::sample_securified_mainnet(
            "Persona",
            6,
            HierarchicalDeterministicFactorInstance::sample_fii10(),
            || {
                GeneralRoleWithHierarchicalDeterministicFactorInstances::r6(HierarchicalDeterministicFactorInstance::sample_id_to_instance(
                    CAP26EntityKind::Identity,
                    Hardened::from_local_key_space(6u32, IsSecurified(true)).unwrap(),
                ))
            },
        );

        assert!(sut.unsecured_controlling_factor_instance().is_none());
    }
}
