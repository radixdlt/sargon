use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct RoleKey {
    pub name: String,
    pub module: ObjectModuleId,
}

impl RoleKey {
    pub fn new(name: String, module: ObjectModuleId) -> Self {
        Self { name, module }
    }

    const DEPOSITOR: &str = "depositor";
    const WITHDRAWER: &str = "withdrawer";

    pub fn main_depositor() -> Self {
        Self::new(Self::DEPOSITOR.to_string(), ObjectModuleId::Main)
    }

    pub fn main_withdrawer() -> Self {
        Self::new(Self::WITHDRAWER.to_string(), ObjectModuleId::Main)
    }
}

impl HasSampleValues for RoleKey {
    fn sample() -> Self {
        Self::main_depositor()
    }

    fn sample_other() -> Self {
        Self::main_withdrawer()
    }
}
