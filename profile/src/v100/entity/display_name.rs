use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(
    sanitize(trim)
    validate(not_empty, max_len = 20)
)]
#[derive(Serialize, Deserialize, Clone, Debug, Display, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayName(String);

impl Default for DisplayName {
    fn default() -> Self {
        Self::new("Unnamed").expect("Default display name")
    }
}
