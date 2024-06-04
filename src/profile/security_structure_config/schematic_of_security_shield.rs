use crate::prelude::*;

decl_security_shield_at_level!(
    /// A "Schematic Of SecurityShield", is at `FactorSourceID` level.
    /// This is what is saved into Profile, being just references to FactorSources.
    /// The user views, creates and manages SecurityShields, which is at `FactorSource`
    /// level.
    Reference,
    FactorSourceID,
    [
        // primary_role_threshold_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_device()),
        FactorSourceID::from(FactorSourceIDFromHash::sample_arculus()),
        FactorSourceID::from(FactorSourceIDFromHash::sample_off_device()),
    ],
    [
        // primary_role_threshold_factors_sample_other
        FactorSourceID::from(FactorSourceIDFromHash::sample_device_other()),
        FactorSourceID::from(FactorSourceIDFromHash::sample_arculus_other()),
        FactorSourceID::from(FactorSourceIDFromHash::sample_off_device_other()),
    ],
    [
        // primary_role_override_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger()),
    ],
    [
        // primary_role_override_factors_sample_other
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger_other()),
    ],
    [
        // recovery_role_threshold_factors_sample
        FactorSourceID::from(
            FactorSourceIDFromAddress::sample_trusted_contact_friend_frank()
        ),
        FactorSourceID::from(
            FactorSourceIDFromAddress::sample_trusted_contact_friend_judy()
        ),
        FactorSourceID::from(
            FactorSourceIDFromAddress::sample_trusted_contact_friend_oscar()
        ),
    ],
    [
        // recovery_role_threshold_factors_sample
        FactorSourceID::from(
            FactorSourceIDFromAddress::sample_trusted_contact_friend_frank()
        ),
        FactorSourceID::from(
            FactorSourceIDFromAddress::sample_trusted_contact_friend_judy()
        ),
    ],
    [
        // recovery_role_override_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger()),
    ],
    [
        // recovery_role_override_factors_sample_other
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger_other()),
    ],
    [
        // confirmation_role_threshold_factors_sample
        FactorSourceID::from(
            FactorSourceIDFromHash::sample_security_questions()
        ),
    ],
    [
        // confirmation_role_threshold_factors_sample_other
        FactorSourceID::from(
            FactorSourceIDFromHash::sample_security_questions_other()
        ),
    ],
    [
        // confirmation_role_override_factors_sample
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger()),
    ],
    [
        // confirmation_role_override_factors_sample_other
        FactorSourceID::from(FactorSourceIDFromHash::sample_ledger_other()),
    ],
);

// pub type SchematicOfSecurityShield = SecurityStructureConfigurationReference;

// impl Identifiable for SecurityStructureConfigurationReference {
//     type ID = <SecurityStructureMetadata as Identifiable>::ID;

//     fn id(&self) -> Self::ID {
//         self.metadata.id()
//     }
// }

// What to call FactorInstance level for SchematicOfSecurityShield? I.e. the collection
// of FactorInstances which is derived from having "used" a SchematicOfSecurityShield for
// a certain account?
// Maybe:
// Shield Ward
// Shield Wield
// Shield Bash
// Raise Shield
// Shield Wielding -> WieldingOfShield

#[cfg(test)]
mod test_security_structure_configuration_reference {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureConfigurationReference;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}

#[cfg(test)]
mod test_role_of_tier_reference {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = RoleOfTierReference;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}

#[cfg(test)]
mod test_security_structure_reference {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = SecurityStructureReference;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }
}
