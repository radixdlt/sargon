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

    pub fn veci(&self) -> VirtualEntityCreatingInstance {
        self.veci.clone()
    }
}

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

    #[allow(clippy::upper_case_acronyms)]
    type SUT = UnsecurifiedEntity;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn network_id() {
        assert_eq!(SUT::sample_other().network_id(), NetworkID::Stokenet);
    }
}
