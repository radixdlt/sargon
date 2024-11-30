use crate::prelude::*;

pub type RoleWithFactorSourceIds<const R: u8> =
    AbstractBuiltRoleWithFactor<R, FactorSourceID>;
