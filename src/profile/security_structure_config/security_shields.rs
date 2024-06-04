use crate::prelude::*;

decl_identified_vec_of!(
    /// A collection of [`SecurityShield`](`SecurityStructureConfigurationReference`s)
    SecurityShield
);

impl HasSampleValues for SecurityShields {
    fn sample() -> Self {
        Self::from_iter([
            SecurityShield::sample(),
            SecurityShield::sample_other(),
        ])
    }
    fn sample_other() -> Self {
        Self::from_iter([SecurityShield::sample_other()])
    }
}
