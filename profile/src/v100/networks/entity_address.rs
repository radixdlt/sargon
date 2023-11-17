use crate::error::Error;

use super::{
    decode_address_helper::decode_address, entity_type::EntityType, network_id::NetworkID,
};

pub trait EntityAddress: Sized {
    fn entity_type() -> EntityType;
    fn with_address_and_network_id(address: &str, network_id: NetworkID) -> Self;
    fn validate(address: &str) {
        assert!(address.starts_with(&Self::entity_type().hrp()))
    }
    fn try_from_bech32(s: &str) -> Result<Self, Error> {
        let (network_id, entity_type, hrp, _) = decode_address(s)?;
        if entity_type != Self::entity_type() {
            return Err(Error::MismatchingEntityTypeWhileDecodingAddress);
        }

        if !hrp.starts_with(&entity_type.hrp()) {
            return Err(Error::MismatchingHRPWhileDecodingAddress);
        }

        return Ok(Self::with_address_and_network_id(s, network_id));
    }
}
