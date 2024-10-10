use crate::prelude::*;

impl HierarchicalDeterministicFactorInstance {
    pub(crate) fn sample_id_to_instance(
        entity_kind: CAP26EntityKind,
        index: HDPathComponent,
    ) -> impl Fn(FactorSourceIDFromHash) -> Self {
        move |id: FactorSourceIDFromHash| {
            Self::sample_mainnet_tx(entity_kind, index, id)
        }
    }

    pub fn sample_mainnet_tx(
        entity_kind: CAP26EntityKind,
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        let derivation_path: DerivationPath = match entity_kind {
            CAP26EntityKind::Account => AccountPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
            CAP26EntityKind::Identity => IdentityPath::new(
                NetworkID::Mainnet,
                CAP26KeyKind::TransactionSigning,
                index.index(),
            )
            .into(),
        };

        let seed = factor_source_id.sample_associated_mnemonic().to_seed();
        let hd_private_key = seed.derive_private_key(&derivation_path);

        HierarchicalDeterministicFactorInstance::new(
            factor_source_id,
            hd_private_key.public_key(),
        )
    }

    pub fn sample_mainnet_tx_account(
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::sample_mainnet_tx(
            CAP26EntityKind::Account,
            index,
            factor_source_id,
        )
    }

    pub fn sample_mainnet_tx_identity(
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::sample_mainnet_tx(
            CAP26EntityKind::Identity,
            index,
            factor_source_id,
        )
    }

    /// 0 | unsecurified | device
    pub fn sample_fi0(entity_kind: CAP26EntityKind) -> Self {
        Self::sample_mainnet_tx(
            entity_kind,
            HDPathComponent::from(0),
            FactorSourceIDFromHash::sample_at(0),
        )
    }

    /// Account: 0 | unsecurified | device
    pub fn sample_fia0() -> Self {
        Self::sample_fi0(CAP26EntityKind::Account)
    }
    /// Identity: 0 | unsecurified | device
    pub fn sample_fii0() -> Self {
        Self::sample_fi0(CAP26EntityKind::Identity)
    }

    /// 1 | unsecurified | ledger
    pub fn sample_fi1(entity_kind: CAP26EntityKind) -> Self {
        Self::sample_mainnet_tx(
            entity_kind,
            HDPathComponent::from(1),
            FactorSourceIDFromHash::sample_at(1),
        )
    }

    /// Account: 1 | unsecurified | ledger
    pub fn sample_fia1() -> Self {
        Self::sample_fi1(CAP26EntityKind::Account)
    }
    /// Identity: 1 | unsecurified | ledger
    pub fn sample_fii1() -> Self {
        Self::sample_fi1(CAP26EntityKind::Identity)
    }

    /// 8 | Unsecurified { Device } (fs10)
    pub fn sample_fi10(entity_kind: CAP26EntityKind) -> Self {
        Self::sample_mainnet_tx(
            entity_kind,
            HDPathComponent::from(8),
            FactorSourceIDFromHash::sample_at(10),
        )
    }

    /// Account: 8 | Unsecurified { Device } (fs10)
    pub fn sample_fia10() -> Self {
        Self::sample_fi10(CAP26EntityKind::Account)
    }

    /// Identity: 8 | Unsecurified { Device } (fs10)
    pub fn sample_fii10() -> Self {
        Self::sample_fi10(CAP26EntityKind::Identity)
    }
}
