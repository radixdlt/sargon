use crate::prelude::*;

/// An enum representing the **category** of a `FactorSource`/`FactorSourceKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FactorSourceCategory {
    /// Something I am.
    Identity,

    /// Something I have.
    Hardware,

    /// Something I know.
    Information,

    /// Some person I trust.
    Contact,

    /// Some institution I trust.
    Custodian,
}

impl FactorSourceCategory {
    fn is_supported(&self) -> bool {
        use FactorSourceCategory::*;
        match self {
            Identity | Hardware | Contact | Information => true,
            Custodian => false,
        }
    }
}
