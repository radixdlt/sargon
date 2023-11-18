use serde::{Deserialize, Serialize};

use super::unsecured_entity_control::UnsecuredEntityControl;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EntitySecurityState {
    Unsecured(UnsecuredEntityControl),
}
