use crate::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RoleKind {
    Primary,
    Recovery,
    Confirmation,
}
