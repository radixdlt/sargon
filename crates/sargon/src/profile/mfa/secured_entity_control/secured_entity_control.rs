use crate::prelude::*;

/// Advanced security control of an entity which has been "securified",
/// meaning an MFA security structure (`SecurityStructureOfFactorSources`)
/// which user has created has been applied to it.
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, derive_more::Debug,
)]
#[serde(rename_all = "camelCase")]
pub struct SecuredEntityControl {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[serde(skip)]
    #[debug(skip)]
    pub __hidden: HiddenConstructor,

    /// Virtual Entity Creation (Factor)Instance
    ///
    /// Optional since if we recovered this SecuredEntityControl part of
    /// account recovery scan we might not know the veci
    pub veci: Option<HierarchicalDeterministicFactorInstance>,

    /// The address of the access controller which controls this entity.
    ///
    /// Looking up the public key (hashes) set in the key-value store at
    /// this address reveals the true factors (public keys) used to protect
    /// this entity. It will be the same as the ones in `security_structure`
    /// if we have not changed them locally, which we should not do unless
    /// we are sure the Ledger corresponds to the values in `security_structure`.
    pub access_controller_address: AccessControllerAddress,

    /// The believed-to-be-current security structure of FactorInstances which
    /// secures this entity.
    pub security_structure: SecurityStructureOfFactorInstances,
}

impl HasFactorInstances for SecuredEntityControl {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance> {
        self.security_structure
            .matrix_of_factors
            .unique_factor_instances()
    }
}

impl SecuredEntityControl {
    /// # Panics
    /// Panics if veci is not unsecurified
    pub fn new(
        veci: impl Into<Option<HierarchicalDeterministicFactorInstance>>,
        access_controller_address: AccessControllerAddress,
        security_structure: SecurityStructureOfFactorInstances,
    ) -> Result<Self> {
        let veci = veci.into();
        if let Some(veci) = veci.clone() {
            if veci.is_securified() {
                panic!("Got securified factor instance as veci, this is a programmer error!")
            }
        };
        Ok(Self {
            __hidden: HiddenConstructor,
            veci,
            access_controller_address,
            security_structure,
        })
    }
}

impl SecuredEntityControl {
    pub fn veci(&self) -> Option<HierarchicalDeterministicFactorInstance> {
        self.veci.clone()
    }
}

impl HasSampleValues for SecuredEntityControl {
    fn sample() -> Self {
        Self::new(
            None,
            AccessControllerAddress::sample(),
            SecurityStructureOfFactorInstances::sample(),
        )
        .unwrap()
    }
    fn sample_other() -> Self {
        Self::new(HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_10_unsecurified_at_index(0), AccessControllerAddress::sample_other(), SecurityStructureOfFactorInstances::sample_other()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecuredEntityControl;

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
    #[should_panic(
        expected = "Got securified factor instance as veci, this is a programmer error!"
    )]
    fn test_panics_if_veci_is_in_securified_space() {
        _ = SUT::new(

            HierarchicalDeterministicFactorInstance::sample_mainnet_account_device_factor_fs_0_securified_at_index(0),
            AccessControllerAddress::sample(),
            SecurityStructureOfFactorInstances::sample(),
        );
    }
}
