use crate::prelude::*;
use radix_engine_common::types::EntityType as EngineEntityType;

/// Type of a wallet Radix Entity - Account or Identity (used by Personas).
///
/// CAP26 uses this type to create separate key spaces for Accounts and Identities
#[derive(
    Serialize, Deserialize, FromRepr, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(u32)] // it is u32 since used in Derivation Paths (CAP26) where each component is a u32.
pub enum AbstractEntityType {
    /// The entity type used by Accounts.
    Account,
    /// The entity type used by Personas.
    Identity,
    /// Resource address
    Resource,
}
impl AbstractEntityType {
    /// Conversion of the Radix Engines type for EntityType to Self.
    pub fn try_from(value: EngineEntityType) -> Result<Self> {
        match value {
            EngineEntityType::GlobalVirtualEd25519Account => Ok(Self::Account),
            EngineEntityType::GlobalVirtualSecp256k1Account => Ok(Self::Account),
            EngineEntityType::GlobalVirtualEd25519Identity => Ok(Self::Identity),
            EngineEntityType::GlobalFungibleResourceManager => Ok(Self::Resource),
            _ => Err(CommonError::UnsupportedEntityType),
        }
    }

    /// Human Readable Part (HRP) used to create account and identity addresses.
    pub fn hrp(&self) -> String {
        match self {
            Self::Account => "account".to_string(),
            Self::Identity => "identity".to_string(),
            Self::Resource => "resource".to_string(),
        }
    }
}
