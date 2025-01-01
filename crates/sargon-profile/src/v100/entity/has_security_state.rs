use crate::prelude::*;

impl<T: HasSecurityState> HasFactorInstances for T {
    fn unique_tx_signing_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_state().unique_tx_signing_factor_instances()
    }
}

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
                if sec_new.access_controller_address
                    != sec_existing.access_controller_address
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

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountOrPersona;

    fn test_set_security_state_fail_cannot_unsecurify(sut: impl Into<SUT>) {
        let mut sut = sut.into();
        assert!(sut.is_securified());

        let unsecurified = EntitySecurityState::sample();
        assert!(unsecurified.is_unsecured());

        let result = sut.set_security_state(unsecurified);
        assert_eq!(
            result,
            Err(CommonError::SecurityStateSecurifiedButExpectedUnsecurified)
        );

        // assert unchanged
        assert!(sut.is_securified());
    }

    #[test]
    fn set_security_state_fail_cannot_unsecurify_account() {
        test_set_security_state_fail_cannot_unsecurify(Account::sample_at(2))
    }

    #[test]
    fn set_security_state_fail_cannot_unsecurify_persona() {
        test_set_security_state_fail_cannot_unsecurify(Persona::sample_at(2))
    }

    fn test_set_security_state_fail_can_change_unsecurified(
        sut: impl Into<SUT>,
    ) {
        let mut sut = sut.into();
        assert!(!sut.is_securified());

        let unsecurified = EntitySecurityState::sample();
        assert!(unsecurified.is_unsecured());

        let result = sut.set_security_state(unsecurified.clone());
        assert!(result.is_ok());
        assert_eq!(sut.security_state(), unsecurified);
    }

    #[test]
    fn set_security_state_fail_can_change_unsecurified_account() {
        test_set_security_state_fail_can_change_unsecurified(Account::sample());
    }

    #[test]
    fn set_security_state_fail_can_change_unsecurified_persona() {
        test_set_security_state_fail_can_change_unsecurified(Persona::sample());
    }

    fn test_set_security_state_fail_access_controller_mismatch(
        sut: impl Into<SUT>,
    ) {
        let mut sut = sut.into();
        let entity_state = sut.security_state();
        assert!(sut.is_securified());

        let other_securified = EntitySecurityState::Securified {
            value: SecuredEntityControl::sample(),
        };

        let result = sut.set_security_state(other_securified);
        assert_eq!(
            result,
            Err(CommonError::SecurityStateAccessControllerAddressMismatch)
        );

        // assert unchanged
        assert_eq!(sut.security_state(), entity_state);
    }

    #[test]
    fn set_security_state_fail_access_controller_mismatch_account() {
        test_set_security_state_fail_access_controller_mismatch(
            Account::sample_at(2),
        );
    }

    #[test]
    fn set_security_state_fail_access_controller_mismatch_persona() {
        test_set_security_state_fail_access_controller_mismatch(
            Persona::sample_at(2),
        )
    }

    fn test_set_security_state_can_change_securified(sut: impl Into<SUT>) {
        let mut sut = sut.into();
        let entity_state = sut.security_state();
        assert!(sut.is_securified());
        let access_controller_address = entity_state
            .clone()
            .as_securified()
            .unwrap()
            .access_controller_address;

        let mut value = SecuredEntityControl::sample();
        value.access_controller_address = access_controller_address;
        let other_securified = EntitySecurityState::Securified { value };

        let result = sut.set_security_state(other_securified);

        assert!(result.is_ok());
        assert!(sut.is_securified());
        assert_ne!(sut.security_state(), entity_state);
    }

    #[test]
    fn set_security_state_can_change_securified_account() {
        test_set_security_state_can_change_securified(Account::sample_at(2));
    }

    #[test]
    fn set_security_state_can_change_securified_persona() {
        test_set_security_state_can_change_securified(Persona::sample_at(2));
    }
}
