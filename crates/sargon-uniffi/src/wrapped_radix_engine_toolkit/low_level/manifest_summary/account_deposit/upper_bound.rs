use crate::prelude::*;
use sargon::UpperBound as InternalUpperBound;

/// Represents an upper bound on a non-negative decimal.
#[derive(Clone, PartialEq, Eq, InternalConversion, uniffi::Enum)]
pub enum UpperBound {
    /// The amount is required to be non-negative before using this model.
    /// This can be validated via [`ManifestResourceConstraint::is_valid_for`].
    Inclusive { decimal: Decimal },

    /// `Unbounded` represents an upper bound above any possible decimal, and is included for
    /// clarity of intention. Considering Decimal has a max size, it is effectively equivalent to
    /// an inclusive bound of [`Decimal::max()`].
    Unbounded,
}
