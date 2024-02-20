use crate::prelude::*;
use radix_engine_common::types::EntityType as ScryptoEntityType;

/// Type of a wallet Radix Entity - Account or Identity (used by Personas).
///
/// CAP26 uses this type to create separate key spaces for Accounts and Identities
#[derive(
    Serialize,
    Deserialize,
    FromRepr,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
)]
#[repr(u32)] // it is u32 since used in Derivation Paths (CAP26) where each component is a u32.
pub enum AbstractEntityType {
    /// The entity type used by Accounts created by the Radix Wallet.
    Account,
    /// The entity type used by Personas.
    Identity,
}

// impl TryFrom<ScryptoEntityType> for AbstractEntityType {
//     type Error = crate::CommonError;

//     fn try_from(value: ScryptoEntityType) -> Result<Self, Self::Error> {
//         match value {
//             ScryptoEntityType::GlobalVirtualEd25519Account => {
//                 Ok(Self::Account)
//             }
//             ScryptoEntityType::GlobalVirtualSecp256k1Account => {
//                 Ok(Self::Account)
//             }
//             ScryptoEntityType::GlobalVirtualEd25519Identity => {
//                 Ok(Self::Identity)
//             }
//             _ => Err(CommonError::UnsupportedEntityType),
//         }
//     }
// }
