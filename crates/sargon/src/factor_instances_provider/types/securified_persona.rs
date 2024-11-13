use crate::prelude::*;

/// The `SecuredEntityControl`, address and possibly third party deposit state of some
/// Securified entity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SecurifiedPersona {
    display_name: DisplayName,
    /// The address which is verified to match the `veci`
    identity_address: IdentityAddress,
    securified_entity_control: SecuredEntityControl,
}
impl HasEntityKind for SecurifiedPersona {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}
impl IsNetworkAware for SecurifiedPersona {
    fn network_id(&self) -> NetworkID {
        self.identity_address.network_id()
    }
}

impl IsSecurifiedEntity for SecurifiedPersona {
    type BaseEntity = Persona;
    fn securified_entity_control(&self) -> SecuredEntityControl {
        self.securified_entity_control()
    }

    fn new(
        name: impl Into<DisplayName>,
        address: IdentityAddress,
        securified_entity_control: SecuredEntityControl,
    ) -> Self {
        Self {
            display_name: name.into(),
            identity_address: address,
            securified_entity_control,
        }
    }
}

impl TryFrom<Persona> for SecurifiedPersona {
    type Error = CommonError;
    fn try_from(value: Persona) -> Result<Self> {
        let securified_entity_control =
            value.security_state.as_securified().cloned().ok_or(
                CommonError::PersonaNotSecurified {
                    address: value.address.to_string(),
                },
            )?;
        Ok(SecurifiedPersona::new(
            value.display_name.clone(),
            value.address.clone(),
            securified_entity_control,
        ))
    }
}

impl TryFrom<AccountOrPersona> for SecurifiedPersona {
    type Error = CommonError;
    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Persona::try_from(value).and_then(SecurifiedPersona::try_from)
    }
}

impl SecurifiedPersona {
    pub fn persona(&self) -> Persona {
        // Persona::new(
        //     self.name.clone(),
        //     self.address(),
        //     EntitySecurityState::Securified(self.securified_entity_control()),
        //     None,
        // )
        todo!()
    }
    pub fn address(&self) -> IdentityAddress {
        self.identity_address.clone()
    }
    pub fn securified_entity_control(&self) -> SecuredEntityControl {
        self.securified_entity_control.clone()
    }
    pub fn third_party_deposit(&self) -> Option<DepositRule> {
        None
    }
}
impl HasSampleValues for SecurifiedPersona {
    fn sample() -> Self {
        // Self::new(
        //     "SecurifiedPersona",
        //     IdentityAddress::sample(),
        //     SecuredEntityControl::sample(),
        //     None,
        // )
        todo!()
    }
    fn sample_other() -> Self {
        // Self::new(
        //     "SecurifiedPersona Other",
        //     IdentityAddress::sample_other(),
        //     SecuredEntityControl::sample_other(),
        //     None,
        // )
        todo!()
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurifiedPersona;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
