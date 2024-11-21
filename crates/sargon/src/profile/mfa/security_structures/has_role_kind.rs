use crate::prelude::*;
pub trait HasRoleKind {
    fn role_kind() -> RoleKind;
}
pub trait HasRoleKindObjectSafe {
    fn get_role_kind(&self) -> RoleKind;
}

impl<T: HasRoleKind> HasRoleKindObjectSafe for T {
    fn get_role_kind(&self) -> RoleKind {
        T::role_kind()
    }
}
