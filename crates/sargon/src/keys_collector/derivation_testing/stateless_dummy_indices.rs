#![cfg(test)]
#![allow(unused)]

use crate::prelude::*;

pub trait SampleDerivationPath: Sized {
    fn hardening_global_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        global_key_space: u32,
    ) -> Self;

    fn hardening_global_index_account_tx(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            global_key_space,
        )
    }

    fn hardening_global_index_account_rola(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::AuthenticationSigning,
            global_key_space,
        )
    }

    fn hardening_global_index_identity_tx(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Identity,
            CAP26KeyKind::TransactionSigning,
            global_key_space,
        )
    }

    fn hardening_global_index_identity_rola(
        network_id: NetworkID,
        global_key_space: u32,
    ) -> Self {
        Self::hardening_global_index(
            network_id,
            CAP26EntityKind::Identity,
            CAP26KeyKind::AuthenticationSigning,
            global_key_space,
        )
    }

    fn unsecurified_hardening_base_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: u32,
    ) -> Self;

    fn account_tx_unsecurified_hardening_base_index(
        network_id: NetworkID,
        index: u32,
    ) -> Self {
        Self::unsecurified_hardening_base_index(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::TransactionSigning,
            index,
        )
    }

    fn for_entity(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        hardened: Hardened,
    ) -> Self;
}
impl SampleDerivationPath for DerivationPath {
    fn for_entity(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        hardened: Hardened,
    ) -> Self {
        match entity_kind {
            CAP26EntityKind::Account => DerivationPath::account(
                AccountPath::new(network_id, key_kind, hardened),
            ),
            CAP26EntityKind::Identity => DerivationPath::identity(
                IdentityPath::new(network_id, key_kind, hardened),
            ),
        }
    }
    fn hardening_global_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        global_key_space: u32,
    ) -> Self {
        let index = Hardened::from_global_key_space(global_key_space).unwrap();
        Self::for_entity(network_id, entity_kind, key_kind, index)
    }
    fn unsecurified_hardening_base_index(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: u32,
    ) -> Self {
        let index = U30::try_from(index).unwrap();
        let index = Hardened::Unsecurified(UnsecurifiedHardened::from(index));
        match entity_kind {
            CAP26EntityKind::Account => DerivationPath::account(
                AccountPath::new(network_id, key_kind, index),
            ),
            CAP26EntityKind::Identity => DerivationPath::identity(
                IdentityPath::new(network_id, key_kind, index),
            ),
        }
    }
}

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
