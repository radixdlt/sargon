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
