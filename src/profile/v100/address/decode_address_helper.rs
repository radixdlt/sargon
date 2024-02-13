use radix_engine_common::types::EntityType as ScryptoEntityType;
use radix_engine_toolkit::functions::address::decode;

use crate::prelude::*;
type EngineDecodeAddressOutput = (u8, ScryptoEntityType, String, [u8; 30]);
pub type DecodeAddressOutput =
    (NetworkID, AbstractEntityType, String, [u8; 30]);

fn engine_decode_address(s: &str) -> Result<EngineDecodeAddressOutput> {
    let Some(tuple) = decode(s) else {
        return Err(CommonError::FailedToDecodeAddressFromBech32(s.to_owned()));
    };
    Ok(tuple)
}

pub fn decode_address(s: &str) -> Result<DecodeAddressOutput> {
    let (network_id_raw, entity_type_engine, hrp, data) =
        engine_decode_address(s)?;
    let entity_type = AbstractEntityType::try_from(entity_type_engine)?;
    let network_id = NetworkID::try_from(network_id_raw)?;
    Ok((network_id, entity_type, hrp, data))
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;

    #[test]
    fn decode_unsupported_entity() {
        assert_eq!(
            decode_address(
                "consensusmanager_rdx1scxxxxxxxxxxcnsmgrxxxxxxxxx000999665565xxxxxxxxxcnsmgr"
            ),
            Err(CommonError::UnsupportedEntityType)
        );
    }
}
