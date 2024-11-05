use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VirtualEntityCreatingInstance {
    /// The instance which as known to have created `address`
    factor_instance: HierarchicalDeterministicFactorInstance,

    /// The address of the entity.
    address: AddressOfAccountOrPersona,
}
impl VirtualEntityCreatingInstance {
    /// # Panics
    /// Panics if factor_instance does not result in address.
    ///
    /// Panics if factor_instance is not in unsecurified space.
    pub fn new(
        factor_instance: HierarchicalDeterministicFactorInstance,
        address: AddressOfAccountOrPersona,
    ) -> Self {
        assert!(
            !factor_instance.is_securified(),
            "factor instance not in unsecurified space"
        );
        assert!(
            address.matches_public_key(factor_instance.public_key()),
            "Discrepancy! PublicKeys does not match, this is a programmer error!"
        );
        Self {
            address,
            factor_instance,
        }
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.address.clone()
    }

    pub fn factor_instance(&self) -> HierarchicalDeterministicFactorInstance {
        self.factor_instance.clone()
    }

    pub fn hd_public_key(&self) -> HierarchicalDeterministicPublicKey {
        self.factor_instance().public_key
    }

    pub fn public_key(&self) -> PublicKey {
        self.hd_public_key().public_key
    }

    fn with_factor_instance_on_network(
        factor_instance: HierarchicalDeterministicFactorInstance,
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
    ) -> Self {
        let public_key = factor_instance.public_key();
        let address = match entity_kind {
            CAP26EntityKind::Account => AddressOfAccountOrPersona::from(
                AccountAddress::new(public_key, network_id),
            ),
            CAP26EntityKind::Identity => AddressOfAccountOrPersona::from(
                IdentityAddress::new(public_key, network_id),
            ),
        };
        Self::new(factor_instance, address)
    }
}

impl HasSampleValues for VirtualEntityCreatingInstance {
    fn sample() -> Self {
        Self::with_factor_instance_on_network(
            HierarchicalDeterministicFactorInstance::sample(),
            CAP26EntityKind::Account,
            NetworkID::Mainnet,
        )
    }
    fn sample_other() -> Self {
        Self::with_factor_instance_on_network(
            HierarchicalDeterministicFactorInstance::sample_other(),
            CAP26EntityKind::Identity,
            NetworkID::Stokenet,
        )
    }
}

#[cfg(test)]
mod test_instance {
    use super::*;

    type Sut = VirtualEntityCreatingInstance;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
