use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractSecurityStructure<FACTOR> {
    /// Metadata of this Security Structure, such as globally unique and
    /// stable identifier, creation date and user chosen label (name).
    pub metadata: SecurityStructureMetadata,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: AbstractMatrixBuilt<FACTOR>,
}

impl<FACTOR> Identifiable for AbstractSecurityStructure<FACTOR> {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

impl<FACTOR: std::cmp::Eq + std::hash::Hash> AbstractSecurityStructure<FACTOR> {
    pub fn all_factors(&self) -> HashSet<&FACTOR> {
        self.matrix_of_factors.all_factors()
    }
}

impl<FACTOR> AbstractSecurityStructure<FACTOR> {
    pub fn with_metadata(
        metadata: SecurityStructureMetadata,
        matrix_of_factors: AbstractMatrixBuilt<FACTOR>,
    ) -> Self {
        Self {
            metadata,
            matrix_of_factors,
        }
    }

    pub fn new(
        display_name: DisplayName,
        matrix_of_factors: AbstractMatrixBuilt<FACTOR>,
    ) -> Self {
        let metadata = SecurityStructureMetadata::new(display_name);
        Self::with_metadata(metadata, matrix_of_factors)
    }
}
