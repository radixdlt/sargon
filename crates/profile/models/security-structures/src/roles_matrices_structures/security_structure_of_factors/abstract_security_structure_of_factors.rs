use crate::prelude::*;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Display,
)]
#[serde(rename_all = "camelCase")]
#[display("{}", self.metadata.display_name)]
pub struct AbstractSecurityStructure<FACTOR> {
    /// Metadata of this Security Structure, such as globally unique and
    /// stable identifier, creation date and user chosen label (name).
    pub metadata: SecurityStructureMetadata,

    /// The structure of factors to use for certain roles, Primary, Recovery
    /// and Confirmation role.
    pub matrix_of_factors: AbstractMatrixBuilt<FACTOR>,

    /// The factor to use for authentication signing aka true Rola Key.
    pub authentication_signing_factor: FACTOR,
}

impl<FACTOR> Identifiable for AbstractSecurityStructure<FACTOR> {
    type ID = <SecurityStructureMetadata as Identifiable>::ID;

    fn id(&self) -> Self::ID {
        self.metadata.id()
    }
}

impl<FACTOR: std::cmp::Eq + std::hash::Hash> AbstractSecurityStructure<FACTOR> {
    pub fn all_factors(&self) -> HashSet<&FACTOR> {
        let mut all = self.matrix_of_factors.all_factors();
        all.extend([&self.authentication_signing_factor]);
        all
    }
}

impl<FACTOR> AbstractSecurityStructure<FACTOR> {
    pub fn with_metadata(
        metadata: SecurityStructureMetadata,
        matrix_of_factors: AbstractMatrixBuilt<FACTOR>,
        authentication_signing_factor: FACTOR,
    ) -> Self {
        Self {
            metadata,
            matrix_of_factors,
            authentication_signing_factor,
        }
    }

    pub fn new(
        display_name: DisplayName,
        matrix_of_factors: AbstractMatrixBuilt<FACTOR>,
        authentication_signing_factor: FACTOR,
    ) -> Self {
        let metadata = SecurityStructureMetadata::new(
            display_name,
            SecurityStructureFlags::new(),
        );
        Self::with_metadata(
            metadata,
            matrix_of_factors,
            authentication_signing_factor,
        )
    }
}

impl<FACTOR> AbstractSecurityStructure<FACTOR> {
    pub fn is_main(&self) -> bool {
        self.metadata.is_main()
    }
}
