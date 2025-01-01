use crate::prelude::*;

pub type RoleWithFactorSourceIds<const ROLE: u8> =
    AbstractBuiltRoleWithFactor<ROLE, FactorSourceID>;
