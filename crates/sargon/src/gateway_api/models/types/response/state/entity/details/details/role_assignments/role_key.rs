use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct RoleKey {
    pub name: String,
}

impl RoleKey {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl HasSampleValues for RoleKey {
    fn sample() -> Self {
        Self::sample_depositor()
    }

    fn sample_other() -> Self {
        Self::sample_withdrawer()
    }
}

impl RoleKey {
    pub fn sample_depositor() -> Self {
        Self::new("depositor".to_string())
    }

    pub fn sample_withdrawer() -> Self {
        Self::new("withdrawer".to_string())
    }
}
