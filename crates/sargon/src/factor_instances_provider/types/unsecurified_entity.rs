use crate::prelude::*;

/// The HierarchicalDeterministicFactorInstance and address of some
/// unsecurified entity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnsecurifiedEntity {
    veci: VirtualEntityCreatingInstance,
}

impl UnsecurifiedEntity {
    pub fn with_veci(veci: VirtualEntityCreatingInstance) -> Self {
        Self { veci }
    }

    /// # Panics
    /// Panics if address does not match `factor_instance`
    pub fn new(
        address: AddressOfAccountOrPersona,
        factor_instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        let veci = VirtualEntityCreatingInstance::new(factor_instance, address);
        Self::with_veci(veci)
    }

    pub fn network_id(&self) -> NetworkID {
        self.address().network_id()
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.veci.clone().address()
    }

    pub fn factor_instance(&self) -> HierarchicalDeterministicFactorInstance {
        self.veci.factor_instance()
    }

    pub fn veci(&self) -> VirtualEntityCreatingInstance {
        self.veci.clone()
    }
}

// impl TryFrom<UnsecurifiedEntity> for AccountOrPersona {
//     type Error = CommonError;
//     fn try_from(value: UnsecurifiedEntity) -> Result<Self> {
//         let address = value.address();
//         let name = "UNNAMED";
//         let uec = UnsecuredEntityControl::new(value.factor_instance(), None)?;
//         let security_state = EntitySecurityState::Unsecured { value: uec };

//         if let Ok(account_address) = address.clone().into_account() {
//             Account::new(name, account_address, security_state).into()
//         } else if let Ok(identity_address) = address.clone().into_identity() {
//             Persona::new(name, identity_address, security_state).into()
//         } else {
//             unreachable!("Either account or persona.")
//         }
//     }
// }

impl HasSampleValues for UnsecurifiedEntity {
    fn sample() -> Self {
        Self::with_veci(VirtualEntityCreatingInstance::sample())
    }
    fn sample_other() -> Self {
        Self::with_veci(VirtualEntityCreatingInstance::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = UnsecurifiedEntity;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }

    // #[test]
    // fn unsecurified_persona_into_tagged_union() {
    //     let sut = Sut::sample_other();
    //     assert!(AccountOrPersona::from(sut).is_persona_entity());
    // }

    // #[test]
    // fn unsecurified_account_into_tagged_union() {
    //     let sut = Sut::sample();
    //     assert!(AccountOrPersona::from(sut).is_account_entity());
    // }

    #[test]
    fn network_id() {
        assert_eq!(Sut::sample_other().network_id(), NetworkID::Stokenet);
    }
}
