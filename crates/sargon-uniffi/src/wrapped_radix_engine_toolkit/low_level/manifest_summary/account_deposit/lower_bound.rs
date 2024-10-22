use crate::prelude::*;
use sargon::LowerBound as InternalLowerBound;

/// Represents a lower bound on a non-negative decimal.
#[derive(Clone, Debug, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum LowerBound {
    /// Represents a lower bound of an infinitesimal amount above 0, and is included for
    /// clarity of intention. Considering Decimal has a limited precision of 10^(-18), it is roughly
    /// equivalent to an inclusive bound of 10^(-18), or Decimal::from_attos(1).
    NonZero,

    /// The amount is required to be non-negative before using this model.
    /// This can be validated via [`ManifestResourceConstraint::is_valid_for`].
    Inclusive { decimal: Decimal },
}