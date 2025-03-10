use crate::prelude::*;
use sargon::RoleKind as InternalRoleKind;

/// A discriminator of a role in a matrix of Factors.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum,
)]
pub enum RoleKind {
    /// The primary role of some matrix of factors
    Primary,
    /// The recovery role of some matrix of factors
    Recovery,
    /// The confirmation role of some matrix of factors
    Confirmation,
}
