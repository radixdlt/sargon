use serde::{Deserialize, Serialize};

use crate::HasSampleValues;

/// A type used to hide a constructor for some other type, use
/// it like this:
///
/// ```rust,ignore
/// pub struct ValidatedName {
///     hiding_ctor: HiddenConstructor,
///     pub name: String,
///     pub name_appended_to_name: String // validated!
/// }
/// ```
///
/// Making it impossible to create `ValidatedName` with invalid value!
///
#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct HiddenConstructor;

