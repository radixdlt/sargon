use crate::prelude::*;

pub trait HasEntityKind {
    fn entity_kind() -> CAP26EntityKind;
}

pub trait HasEntityKindObjectSafe {
    fn get_entity_kind(&self) -> CAP26EntityKind;
}

impl<T: HasEntityKind> HasEntityKindObjectSafe for T {
    fn get_entity_kind(&self) -> CAP26EntityKind {
        T::entity_kind()
    }
}

pub trait NewEntityPath: Sized {
    fn new(
        network_id: impl Into<NetworkID>,
        key_kind: impl Into<CAP26KeyKind>,
        index: impl Into<Hardened>,
    ) -> Self;

    fn new_mainnet_transaction_signing(index: impl Into<Hardened>) -> Self {
        Self::new(NetworkID::Mainnet, CAP26KeyKind::TransactionSigning, index)
    }
}

pub trait NewEntityPathCheckingEntityKind: NewEntityPath {
    fn try_from_unvalidated(path: UnvalidatedCAP26Path) -> Result<Self>;
}

impl<T: HasEntityKind + NewEntityPath> NewEntityPathCheckingEntityKind for T {
    fn try_from_unvalidated(path: UnvalidatedCAP26Path) -> Result<Self> {
        let entity_kind = path.entity_kind;
        if entity_kind != Self::entity_kind() {
            return Err(CommonError::WrongEntityKind {
                expected: Self::entity_kind().to_string(),
                found: entity_kind.to_string(),
            });
        }
        Ok(Self::new(path.network_id, path.key_kind, path.index))
    }
}
