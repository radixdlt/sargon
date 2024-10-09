use crate::prelude::*;
use sargon::AccountPath as InternalAccountPath;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
     uniffi::Record,
)]
pub struct AccountPath {
    pub path: HDPath,

    pub network_id: NetworkID,

    pub entity_kind: CAP26EntityKind,

    pub key_kind: CAP26KeyKind,

    pub index: HDPathValue,
}

impl From<InternalAccountPath> for AccountPath {
    fn from(value: InternalAccountPath) -> Self {
        Self {
            path: value.path.into(),
            network_id: value.network_id.into(),
            entity_kind: value.entity_kind.into(),
            key_kind: value.key_kind.into(),
            index: value.index.into(),
        }
    }
}

impl Into<InternalAccountPath> for AccountPath {
    fn into(self) -> InternalAccountPath {
        InternalAccountPath {
            path: self.path.into(),
            network_id: self.network_id.into(),
            entity_kind: self.entity_kind.into(),
            key_kind: self.key_kind.into(),
            index: self.index.into(),
        }
    }
}


#[uniffi::export]
pub fn new_account_path_sample() -> AccountPath {
    InternalAccountPath::sample().into()
}

#[uniffi::export]
pub fn new_account_path_sample_other() -> AccountPath {
    InternalAccountPath::sample_other().into()
}

#[uniffi::export]
pub fn new_account_path(
    network_id: NetworkID,
    key_kind: CAP26KeyKind,
    index: HDPathValue,
) -> AccountPath {
    InternalAccountPath::new(network_id.into(), key_kind.into(), index.into()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = AccountPath;

    #[test]
    fn hash_of_samples() {
        assert_eq!(
            HashSet::<SUT>::from_iter([
                new_account_path_sample(),
                new_account_path_sample_other(),
                // duplicates should get removed
                new_account_path_sample(),
                new_account_path_sample_other(),
            ])
            .len(),
            2
        );
    }
}
