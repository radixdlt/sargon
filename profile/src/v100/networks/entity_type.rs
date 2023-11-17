use std::fmt::Display;

use radix_engine_common::types::EntityType as EngineEntityType;

use crate::error::Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
    Account,
    Identity,
}
impl EntityType {
    pub fn do_try_from(value: EngineEntityType) -> Result<Self, Error> {
        match value {
            EngineEntityType::GlobalVirtualEd25519Account => Ok(Self::Account),
            EngineEntityType::GlobalVirtualSecp256k1Account => Ok(Self::Account),
            EngineEntityType::GlobalVirtualEd25519Identity => Ok(Self::Identity),
            EngineEntityType::GlobalVirtualSecp256k1Identity => Ok(Self::Identity),
            _ => Err(Error::UnsupportedEntityType),
        }
    }

    pub fn hrp(&self) -> String {
        match self {
            Self::Account => "account".to_string(),
            Self::Identity => "identity".to_string(),
        }
    }
}

impl Display for EntityType {
    fn fmt(
        &self,
        f: &mut radix_engine_common::prelude::fmt::Formatter<'_>,
    ) -> radix_engine_common::prelude::fmt::Result {
        write!(f, "{}", self.hrp())
    }
}
