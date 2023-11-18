use radix_engine_common::types::EntityType as EngineEntityType;
use radix_engine_toolkit_core::functions::address::decode;

use crate::{error::Error, v100::{networks::network::network_id::NetworkID, entity::entity_type::EntityType}};

type EngineDecodeAddressOutput = (u8, EngineEntityType, String, [u8; 30]);
pub type DecodeAddressOutput = (NetworkID, EntityType, String, [u8; 30]);

fn engine_decode_address(s: &str) -> Result<EngineDecodeAddressOutput, Error> {
    let Some(tuple) = decode(&s) else {
        return Err(Error::FailedToDecodeAddressFromBech32);
    };
    Ok(tuple)
}

pub fn decode_address(s: &str) -> Result<DecodeAddressOutput, Error> {
    let (network_id_raw, entity_type_engine, hrp, data) = engine_decode_address(s)?;
    let entity_type = EntityType::try_from(entity_type_engine)?;
    let network_id = NetworkID::try_from(network_id_raw)?;
    return Ok((network_id, entity_type, hrp, data));
}
