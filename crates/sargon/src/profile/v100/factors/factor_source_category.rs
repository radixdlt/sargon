use crate::prelude::*;

/// An enum representing the **category** of a `FactorSource`/`FactorSourceKind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactorSourceCategory {
    /// Something I am.
    Identity,

    /// Something I have.
    Hardware,

    /// Something I know.
    Information,

    /// Someone else.
    Contact,
}
