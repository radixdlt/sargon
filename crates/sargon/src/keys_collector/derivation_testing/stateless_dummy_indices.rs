#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;
use crate::DerivationPathConstructors;

#[derive(Default, Clone, Debug)]
pub(crate) struct StatelessDummyIndices;

impl StatelessDummyIndices {
    pub(crate) fn next_derivation_index_for(
        &self,
        key_space: KeySpace,
    ) -> HDPathComponent {
        HDPathComponent::from_local_key_space(0, key_space).unwrap()
    }

    pub(crate) fn next_derivation_path(
        &self,
        network_id: NetworkID,
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> DerivationPath {
        let index = self.next_derivation_index_for(key_space);
        let hardened = Hardened::try_from(index).unwrap();
        DerivationPath::for_entity(network_id, entity_kind, key_kind, hardened)
    }
}
