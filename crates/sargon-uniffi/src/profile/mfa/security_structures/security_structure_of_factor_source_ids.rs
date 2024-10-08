use crate::prelude::*;

decl_security_structure_of!(
    /// A security structure at FactorSourceID level, this is
    /// what is serialized and store into Profile, we convert
    /// into this structure from `SecurityStructureOfFactorSources`.
    FactorSourceID,
);
