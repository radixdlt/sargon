use crate::prelude::*;

decl_security_structure_config!(
    /// Apa
    Reference,
    FactorSourceID,
    [
        // primary_role_threshold_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_device()),
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger()),
    ],
    [
        // primary_role_threshold_factors_sample_other
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger())
    ],
    [
        // primary_role_super_admin_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger()),
    ],
    [
        // primary_role_super_admin_factors_sample_other
    ],
    [
        // confirmation_role_threshold_factors_sample
    ],
    [
        // confirmation_role_threshold_factors_sample_other   
    ],
    [
        // confirmation_role_super_admin_factors_sample
    ],
    [
        // confirmation_role_super_admin_factors_sample_other
    ],
    [
        // recovery_role_threshold_factors_sample
    ],
    [
        // recovery_role_threshold_factors_sample_other
    ],
    [
        // recovery_role_super_admin_factors_sample
    ],
    [
        // recovery_role_super_admin_factors_sample_other
    ]
);

// #[cfg(test)]
// mod test_security_structure_configuration_reference {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = SecurityStructureConfigurationReference;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }
// }

// #[cfg(test)]
// mod test_role_of_tier_reference {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = RoleOfTierReference;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }
// }

// #[cfg(test)]
// mod test_security_structure_reference {
//     use super::*;

//     #[allow(clippy::upper_case_acronyms)]
//     type SUT = SecurityStructureReference;

//     #[test]
//     fn equality() {
//         assert_eq!(SUT::sample(), SUT::sample());
//         assert_eq!(SUT::sample_other(), SUT::sample_other());
//     }

//     #[test]
//     fn inequality() {
//         assert_ne!(SUT::sample(), SUT::sample_other());
//     }
// }
