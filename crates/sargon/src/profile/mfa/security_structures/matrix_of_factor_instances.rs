use crate::prelude::*;

impl MatrixOfFactorInstances {
    pub fn fulfilling_matrix_of_factor_sources_with_instances(
        consuming_instances: &mut IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self> {
        let instances = &consuming_instances.clone();
        println!("🐁 fulfilling_matrix_of_factor_sources_with_instances:\nmatrix_of_factor_sources: {:?}\n\ninstances to consume: {:?}\n", matrix_of_factor_sources, instances);
        let primary = fulfilling_role_of_factor_sources_with_factor_instances(
            instances,
            &matrix_of_factor_sources,
            PrimaryRoleWithFactorInstances::new,
        )?;
        let recovery = fulfilling_role_of_factor_sources_with_factor_instances(
            instances,
            &matrix_of_factor_sources,
            RecoveryRoleWithFactorInstances::new,
        )?;
        let confirmation =
            fulfilling_role_of_factor_sources_with_factor_instances(
                instances,
                &matrix_of_factor_sources,
                ConfirmationRoleWithFactorInstances::new,
            )?;

        let matrix = Self::new(primary, recovery, confirmation)?;

        // Now that we have assigned instances, **possible the SAME INSTANCE to multiple roles**,
        // lets delete them from the consuming_instances map.
        for instance in matrix.all_factors() {
            let fsid =
                &FactorSourceIDFromHash::try_from(instance.factor_source_id)
                    .unwrap();
            let existing = consuming_instances.get_mut(fsid).unwrap();

            let to_remove = HierarchicalDeterministicFactorInstance::try_from(
                instance.clone(),
            )
            .unwrap();

            existing.shift_remove(&to_remove);

            if existing.is_empty() {
                println!(
                    "🧹🧹🧹 pruning {:?}, since no more instances were found",
                    fsid
                );
                consuming_instances.shift_remove_entry(fsid);
            }
        }

        Ok(matrix)
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
pub trait RoleWithFactors<Factor: std::cmp::Eq + std::hash::Hash> {
    fn get_threshold_factors(&self) -> &Vec<Factor>;
    fn get_threshold(&self) -> u8;
    fn get_override_factors(&self) -> &Vec<Factor>;

    fn all_factors(&self) -> HashSet<&Factor> {
        let mut factors =
            HashSet::from_iter(self.get_threshold_factors().iter());
        factors.extend(self.get_override_factors().iter());
        factors
    }
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

// TODO: MFA - Upgrade this method to follow the rules of when a factor instance might
// be used by MULTIPLE roles. This is a temporary solution to get the tests to pass.
// A proper solution should use follow the rules laid out in:
// https://radixdlt.atlassian.net/wiki/spaces/AT/pages/3758063620/MFA+Rules+for+Factors+and+Security+Shields
fn fulfilling_role_of_factor_sources_with_factor_instances<
    U: HasRoleKind + RoleWithFactors<FactorInstance>,
>(
    consuming_instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    matrix_of_factor_sources: &MatrixOfFactorSources,
    make_role: impl FnOnce(
        Vec<FactorInstance>,
        u8,
        Vec<FactorInstance>,
    ) -> Result<U>,
) -> Result<U> {
    let role_kind = U::role_kind();
    let role_of_sources = matrix_of_factor_sources.get_role_of_kind(role_kind);
    let threshold: u8 = role_of_sources.get_threshold();

    let threshold_factor_instances =
        try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
            consuming_instances,
            role_of_sources.get_threshold_factors()
        )?;

    let override_factor_instances =
    try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
        consuming_instances,
        role_of_sources.get_override_factors()
    )?;

    let role = make_role(
        threshold_factor_instances,
        threshold,
        override_factor_instances,
    )?;

    assert_eq!(role.get_role_kind(), role_kind);
    Ok(role)
}

fn try_filling_factor_list_of_role_of_factor_sources_with_factor_instances(
    instances: &IndexMap<FactorSourceIDFromHash, FactorInstances>,
    from: &Vec<FactorSource>,
) -> Result<Vec<FactorInstance>> {
    from.iter()
        .map(|f| {
            if let Some(existing) = instances.get(&f.id_from_hash()) {
                assert!(!existing.is_empty());
                println!("🐙 existing: {:?}", existing,);
                let hd_instance = existing.first().ok_or(
                    CommonError::MissingFactorMappingInstancesIntoRole,
                )?;
                let instance = FactorInstance::from(hd_instance);
                Ok(instance)
            } else {
                println!("🐙 ❌ MISSING! factor: {:?}", f.id_from_hash(),);
                Err(CommonError::MissingFactorMappingInstancesIntoRole)
            }
        })
        .collect::<Result<Vec<FactorInstance>>>()
}
