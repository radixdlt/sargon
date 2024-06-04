use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SchematicOfSecurityShield`](`SecurityStructureConfigurationReference`s)
    SchematicOfSecurityShield
);

impl HasSampleValues for SchematicOfSecurityShields {
    fn sample() -> Self {
        Self::from_iter([
            SchematicOfSecurityShield::sample(),
            SchematicOfSecurityShield::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SchematicOfSecurityShield::sample_other()])
    }
}
