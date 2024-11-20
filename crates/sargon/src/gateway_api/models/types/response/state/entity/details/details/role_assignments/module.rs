use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
pub enum ObjectModuleId {
    Main,
    Metadata,
    Royalty,
    RoleAssignment,
}
