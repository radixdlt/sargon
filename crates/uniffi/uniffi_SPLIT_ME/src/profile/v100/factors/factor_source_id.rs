use crate::prelude::*;
use sargon::FactorSourceID as InternalFactorSourceID;

/// A unique and stable identifier of a FactorSource, e.g. a
/// DeviceFactorSource being a mnemonic securely stored in a
/// device (phone), where the ID of it is the hash of a special
/// key derived near the root of it.
#[derive(Clone, PartialEq, Eq, Hash, uniffi::Enum)]
pub enum FactorSourceID {
    /// FactorSourceID from the blake2b hash of the special HD public key derived at `CAP26::GetID`,
    /// for a certain `FactorSourceKind`
    Hash { value: FactorSourceIDFromHash },
}

delegate_display_debug_into!(FactorSourceID, InternalFactorSourceID);

json_data_convertible!(FactorSourceID);

impl FactorSourceID {
    pub fn into_internal(&self) -> InternalFactorSourceID {
        self.clone().into()
    }
}

impl From<InternalFactorSourceID> for FactorSourceID {
    fn from(value: InternalFactorSourceID) -> Self {
        match value {
            InternalFactorSourceID::Hash { value } => FactorSourceID::Hash {
                value: value.into(),
            },
            InternalFactorSourceID::Address { value: _ } => {
                panic!("Address not yet supported in the Wallet")
            }
        }
    }
}

impl From<FactorSourceID> for InternalFactorSourceID {
    fn from(val: FactorSourceID) -> Self {
        match val {
            FactorSourceID::Hash { value } => InternalFactorSourceID::Hash {
                value: value.into(),
            },
        }
    }
}

#[uniffi::export]
pub fn factor_source_id_to_string(factor_source_id: FactorSourceID) -> String {
    factor_source_id.into_internal().to_string()
}

#[uniffi::export]
pub fn new_factor_source_id_sample() -> FactorSourceID {
    InternalFactorSourceID::sample().into()
}

#[uniffi::export]
pub fn new_factor_source_id_sample_other() -> FactorSourceID {
    InternalFactorSourceID::sample_other().into()
}
