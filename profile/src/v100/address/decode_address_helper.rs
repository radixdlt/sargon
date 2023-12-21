use radix_engine_common::types::EntityType as EngineEntityType;
use radix_engine_toolkit::functions::address::decode;
use wallet_kit_common::NetworkID;

use wallet_kit_common::CommonError as Error;

use crate::v100::AbstractEntityType;

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

#[cfg(test)]
mod tests {

    use super::decode_address;
    use wallet_kit_common::CommonError as Error;

    #[test]
    fn decode_unsupported_entity() {
        assert_eq!(
            decode_address(
                "consensusmanager_rdx1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxcnsmgr"
            ),
            Err(Error::UnsupportedEntityType)
        );
    }
}
