use crate::prelude::*;

impl<const ROLE: u8> From<RoleWithFactorInstances<ROLE>> for ScryptoAccessRule {
    fn from(value: RoleWithFactorInstances<ROLE>) -> Self {
        let from_factors =
            |factors: &Vec<FactorInstance>| -> Vec<ScryptoResourceOrNonFungible> {
                factors
                    .iter()
                    .map(|instance| instance.badge.clone())
                    .map(ScryptoResourceOrNonFungible::from)
                    .collect()
            };
        ScryptoAccessRule::Protected(ScryptoCompositeRequirement::AnyOf(vec![
            ScryptoCompositeRequirement::BasicRequirement(
                ScryptoBasicRequirement::CountOf(
                    value.get_threshold(),
                    from_factors(value.get_threshold_factors()),
                ),
            ),
            ScryptoCompositeRequirement::BasicRequirement(
                ScryptoBasicRequirement::AnyOf(from_factors(
                    value.get_override_factors(),
                )),
            ),
        ]))
    }
}

impl From<MatrixOfFactorInstances> for ScryptoRuleSet {
    fn from(
        MatrixOfFactorInstances {
            primary_role,
            recovery_role,
            confirmation_role,
            ..
        }: MatrixOfFactorInstances,
    ) -> Self {
        Self {
            primary_role: primary_role.into(),
            recovery_role: recovery_role.into(),
            confirmation_role: confirmation_role.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_minutes_per_day() {
        assert_eq!(MINUTES_PER_DAY, 1440);
    }
}
