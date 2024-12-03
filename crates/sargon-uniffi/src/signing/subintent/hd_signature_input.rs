use crate::prelude::*;
use sargon::HDSignatureInput as InternalHDSignatureInput;
use sargon::SubintentHash as InternalSubintentHash;
type InternalHDSignatureInputForSubintent =
    InternalHDSignatureInput<InternalSubintentHash>;

#[derive(Clone, PartialEq, Eq, Hash, uniffi::Record)]
pub struct HDSignatureInputForSubintent {
    /// Hash which was signed.
    pub subintent_hash: SubintentHash,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}

impl HDSignatureInputForSubintent {
    pub fn into_internal(&self) -> InternalHDSignatureInputForSubintent {
        self.clone().into()
    }
}

impl From<InternalHDSignatureInputForSubintent>
    for HDSignatureInputForSubintent
{
    fn from(value: InternalHDSignatureInputForSubintent) -> Self {
        Self {
            subintent_hash: value.payload_id.into(),
            owned_factor_instance: value.owned_factor_instance.into(),
        }
    }
}

impl From<HDSignatureInputForSubintent>
    for InternalHDSignatureInputForSubintent
{
    fn from(value: HDSignatureInputForSubintent) -> Self {
        Self::new(
            value.subintent_hash.into_internal(),
            value.owned_factor_instance.into_internal(),
        )
    }
}

decl_conversion_tests_for!(HDSignatureInputForSubintent);
