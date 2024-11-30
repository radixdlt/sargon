use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbstractSecurityStructure<F> {
    /// Metadata of this Security Structure, such as globally unique and
    /// stable identifier, creation date and user chosen label (name).
    pub metadata: SecurityStructureMetadata,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: AbstractMatrixBuilt<F>,
}

impl<F> Identifiable for AbstractSecurityStructure<F> {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

impl<F: std::cmp::Eq + std::hash::Hash> AbstractSecurityStructure<F> {
    pub fn all_factors(&self) -> HashSet<&F> {
        self.matrix_of_factors.all_factors()
    }
}

impl<F> AbstractSecurityStructure<F> {
    pub fn with_metadata(
        metadata: SecurityStructureMetadata,
        matrix_of_factors: AbstractMatrixBuilt<F>,
    ) -> Self {
        Self {
            metadata,
            matrix_of_factors,
        }
    }

    pub fn new(
        display_name: DisplayName,
        matrix_of_factors: AbstractMatrixBuilt<F>,
    ) -> Self {
        let metadata = SecurityStructureMetadata::new(display_name);
        Self::with_metadata(metadata, matrix_of_factors)
    }
}
