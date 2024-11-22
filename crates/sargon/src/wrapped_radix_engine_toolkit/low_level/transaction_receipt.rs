use crate::prelude::*;

pub(crate) fn decode_engine_toolkit_receipt(
    receipt: ScryptoSerializableToolkitTransactionReceipt, 
    network_id: NetworkID
) -> Result<ScryptoRuntimeToolkitTransactionReceipt> {
receipt
.into_runtime_receipt(&ScryptoAddressBech32Decoder::new(
    &network_id.network_definition(),
))
.map_err(|e| {
    error!("Failed to decode engine toolkit receipt  {:?}", e);
    CommonError::FailedToDecodeEngineToolkitReceipt
})
}