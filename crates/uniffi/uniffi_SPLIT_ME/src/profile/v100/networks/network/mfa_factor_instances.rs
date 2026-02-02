use crate::prelude::*;
use sargon::MFAFactorInstances as InternalMFAFactorInstances;

#[uniffi::export]
pub fn new_mfa_factor_instances_from_json_bytes(
    json_bytes: &BagOfBytes,
) -> Result<Vec<MFAFactorInstance>> {
    json_bytes
        .to_vec()
        .deserialize::<InternalMFAFactorInstances>()
        .into_iter_result()
}

#[uniffi::export]
pub fn mfa_factor_instances_to_json_bytes(
    mfa_factor_instances: Vec<MFAFactorInstance>,
) -> BagOfBytes {
    let internal: InternalMFAFactorInstances =
        mfa_factor_instances.into_internal();
    let bytes = internal.serialize_to_bytes().unwrap();
    bytes.into()
}
