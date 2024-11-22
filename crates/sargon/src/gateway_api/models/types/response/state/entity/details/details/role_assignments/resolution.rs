use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug, EnumAsInner)]
pub enum RoleAssignmentResolution {
    Explicit,
    Owner,
}
