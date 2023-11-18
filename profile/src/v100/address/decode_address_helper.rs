use radix_engine_common::types::EntityType as EngineEntityType;
use radix_engine_toolkit_core::functions::address::decode;
use wallet_kit_test_utils::error::Error;

use crate::v100::{
    entity::abstract_entity_type::AbstractEntityType, networks::network::network_id::NetworkID,
};

type EngineDecodeAddressOutput = (u8, EngineEntityType, String, [u8; 30]);
pub type DecodeAddressOutput = (NetworkID, AbstractEntityType, String, [u8; 30]);

fn engine_decode_address(s: &str) -> Result<EngineDecodeAddressOutput, Error> {
    let Some(tuple) = decode(&s) else {
        return Err(Error::FailedToDecodeAddressFromBech32);
    };
    Ok(tuple)
}

pub fn decode_address(s: &str) -> Result<DecodeAddressOutput, Error> {
    let (network_id_raw, entity_type_engine, hrp, data) = engine_decode_address(s)?;
    let entity_type = AbstractEntityType::try_from(entity_type_engine)?;
    let network_id = NetworkID::try_from(network_id_raw)?;
    return Ok((network_id, entity_type, hrp, data));
}
