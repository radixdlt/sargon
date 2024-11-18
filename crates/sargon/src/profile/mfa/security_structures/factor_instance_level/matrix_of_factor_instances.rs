use sbor::prelude::indexmap::IndexSet;

use crate::prelude::*;

impl HasFactorInstances for MatrixOfFactorInstances {
    fn unique_factor_instances(&self) -> IndexSet<FactorInstance> {
        let mut set = IndexSet::new();
        set.extend(self.primary_role.all_factors().into_iter().cloned());
        set.extend(self.recovery_role.all_factors().into_iter().cloned());
        set.extend(self.confirmation_role.all_factors().into_iter().cloned());
        set
    }
}

impl HasSampleValues for MatrixOfFactorInstances {
    fn sample() -> Self {
        Self::new(
            PrimaryRoleWithFactorInstances::sample(),
            RecoveryRoleWithFactorInstances::sample(),
            ConfirmationRoleWithFactorInstances::sample(),
        )
        .unwrap()
    }

    fn sample_other() -> Self {
        Self::new(
            PrimaryRoleWithFactorInstances::sample_other(),
            RecoveryRoleWithFactorInstances::sample_other(),
            ConfirmationRoleWithFactorInstances::sample_other(),
        )
        .unwrap()
    }
}

impl MatrixOfFactorInstances {
    /// Maps `MatrixOfFactorSources -> MatrixOfFactorInstances` by
    /// "assigning" FactorInstances to each MatrixOfFactorInstances from
    /// `consuming_instances`.
    ///
    /// NOTE:
    /// **One FactorInstance might be used multiple times in the MatrixOfFactorInstances,
    /// e.g. ones in the PrimaryRole(WithFactorInstances) and again in RecoveryRole(WithFactorInstances) or
    /// in RecoveryRole(WithFactorInstances)**.
    ///
    /// However, the same FactorInstance is NEVER used in two different MatrixOfFactorInstances.
    ///
    ///
    pub fn fulfilling_matrix_of_factor_sources_with_instances(
        consuming_instances: &mut IndexMap<
            FactorSourceIDFromHash,
            FactorInstances,
        >,
        matrix_of_factor_sources: MatrixOfFactorSources,
    ) -> Result<Self> {
        let instances = &consuming_instances.clone();

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

        // Now that we have assigned instances, **possibly the SAME INSTANCE to multiple roles**,
        // lets delete them from the `consuming_instances` map.
        for instance in matrix.all_factors() {
            let fsid =
                &FactorSourceIDFromHash::try_from(instance.factor_source_id)
                    .unwrap();
            let existing = consuming_instances.get_mut(fsid).unwrap();

            let to_remove = HierarchicalDeterministicFactorInstance::try_from(
                instance.clone(),
            )
            .unwrap();

            // We remove at the beginning of the list first.
            existing.shift_remove(&to_remove);

            if existing.is_empty() {
                // not needed per se, but feels prudent to "prune".
                consuming_instances.shift_remove_entry(fsid);
            }
        }

        Ok(matrix)
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
    from: &[FactorSource],
) -> Result<Vec<FactorInstance>> {
    from.iter()
        .map(|f| {
            if let Some(existing) = instances.get(&f.id_from_hash()) {
                let hd_instance = existing.first().ok_or(
                    CommonError::MissingFactorMappingInstancesIntoRole,
                )?;
                let instance = FactorInstance::from(hd_instance);
                Ok(instance)
            } else {
                Err(CommonError::MissingFactorMappingInstancesIntoRole)
            }
        })
        .collect::<Result<Vec<FactorInstance>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = MatrixOfFactorInstances;

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
    fn err_if_no_instance_found_for_factor_source() {
        assert!(matches!(
            SUT::fulfilling_matrix_of_factor_sources_with_instances(
                &mut IndexMap::new(),
                MatrixOfFactorSources::sample()
            ),
            Err(CommonError::MissingFactorMappingInstancesIntoRole)
        ));
    }

    #[test]
    fn err_if_empty_instance_found_for_factor_source() {
        assert!(matches!(
            SUT::fulfilling_matrix_of_factor_sources_with_instances(
                &mut IndexMap::kv(
                    FactorSource::sample_device_babylon().id_from_hash(),
                    FactorInstances::from_iter([])
                ),
                MatrixOfFactorSources::sample()
            ),
            Err(CommonError::MissingFactorMappingInstancesIntoRole)
        ));
    }
}
