use crate::prelude::*;

use super::decl_matrix_macro::matrix_conversion;

matrix_conversion!(
    /// Matrix of `FactorInstance`s containing the primary, recovery, and confirmation roles with `FactorInstance`s
    FactorInstance
);
