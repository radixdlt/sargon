use crate::prelude::*;

impl MatrixOfFactorInstances {
    pub fn fulfilling_matrix_of_factor_sources_with_instances(
        instances: &mut IndexMap<FactorSourceIDFromHash, FactorInstances>,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self> {
        let primary = fulfilling_role_of_factor_sources_with_factor_instances::<
            PrimaryRoleWithFactorInstances,
        >(
            RoleKind::Primary,
            instances,
            &matrix_of_factor_sources,
            PrimaryRoleWithFactorInstances::new,
        )?;
        // let recovery = fulfilling_role_of_factor_sources_with_factor_instances(
        //     RoleKind::Recovery,
        //     instances,
        //     matrix_of_factor_sources,
        //     RecoveryRoleWithFactorInstances::new,
        // )?;
        // let confirmation =
        //     fulfilling_role_of_factor_sources_with_factor_instances(
        //         RoleKind::Confirmation,
        //         instances,
        //         matrix_of_factor_sources,
        //         ConfirmationRoleWithFactorInstances::new,
        //     )?;
        // Self::new(primary, recovery, confirmation)
        todo!()
    }
}

pub trait HasRoleKind {
    fn role_kind() -> RoleKind;
}
impl HasRoleKind for PrimaryRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Primary
    }
}
impl HasRoleKind for RecoveryRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Recovery
    }
}
impl HasRoleKind for ConfirmationRoleWithFactorInstances {
    fn role_kind() -> RoleKind {
        RoleKind::Confirmation
    }
}

pub trait HasRoleKindObjectSafe {
    fn get_role_kind(&self) -> RoleKind;
}
pub trait RoleWithFactors<Factor> {
    fn get_threshold_factors(&self) -> Vec<Factor>;
    fn get_threshold(&self) -> u8;
    fn get_override_factors(&self) -> Vec<Factor>;
}

impl<T: HasRoleKind> HasRoleKindObjectSafe for T {
    fn get_role_kind(&self) -> RoleKind {
        T::role_kind()
    }
}
impl HasRoleKindObjectSafe
    for GeneralRoleWithHierarchicalDeterministicFactorInstances
{
    fn get_role_kind(&self) -> RoleKind {
        self.role
    }
}

fn fulfilling_role_of_factor_sources_with_factor_instances<
    U: HasRoleKindObjectSafe + RoleWithFactors<FactorInstance>,
>(
    role_kind: RoleKind,
    instances: &mut IndexMap<FactorSourceIDFromHash, FactorInstances>,
    matrix_of_factor_sources: &MatrixOfFactorSources,
    make_role: impl FnOnce(
        Vec<FactorInstance>,
        u8,
        Vec<FactorInstance>,
    ) -> Result<U>,
) -> Result<U> {
    let role_of_sources = matrix_of_factor_sources.get_role_of_kind(role_kind);
    // let threshold_factors: Vec<FactorInstance> =
    //     role_of_sources.get_threshold_factors();
    // let threshold: u8 = role_of_sources.get_threshold();
    // let override_factors: Vec<FactorInstance> =
    //     role_of_sources.get_override_factors();
    // let role = make_role(threshold_factors, threshold, override_factors)?;
    // assert_eq!(role.get_role_kind(), role_kind);
    // Ok(role)
    todo!()
}

/* impl RoleWithFactors {
    pub fn fulfilling_matrix_of_factor_sources_with_instances(
    role: RoleKind,
        instances: &mut IndexMap<FactorSourceIDFromHash, FactorInstances>,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self> {
        let mut get_factors =
            |required: Vec<FactorSource>| -> Result<Vec<HierarchicalDeterministicFactorInstance>> {
                required
                    .iter()
                    .map(|f| {
                        if let Some(existing) = instances
                        .get_mut(&f.id_from_hash()) {
                            assert!(!existing.is_empty());
                            let instance = existing.shift_remove_index(0);
                            Ok(instance)
                        } else {
                            Err(CommonError::MissingFactorMappingInstancesIntoRole)
                        }
                        })
                    .collect::<Result<Vec<HierarchicalDeterministicFactorInstance>>>()
            };

        let threshold_factors = get_factors(matrix_of_factor_sources.threshold_factors)?;
        let override_factors = get_factors(matrix_of_factor_sources.override_factors)?;

        Self::new(
            threshold_factors,
            matrix_of_factor_sources.threshold,
            override_factors,
        )
    }
}
    */
