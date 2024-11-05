use std::{any::TypeId, hash::Hash};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssertMatches {
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub entity_kind: CAP26EntityKind,
    pub key_space: KeySpace,
}
impl AssertMatches {
    pub fn matches(&self, path: &DerivationPath) -> DerivationPath {
        assert_eq!(self.entity_kind, path.get_entity_kind());
        assert_eq!(self.network_id, path.network_id());
        assert_eq!(self.key_kind, path.get_key_kind());
        assert_eq!(self.key_space, path.key_space());
        path.clone()
    }
}
trait HighestDerivationPathIndex {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent>;
}
impl HighestDerivationPathIndex for MatrixOfFactorInstances {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        // self.all_factors()
        //     .into_iter()
        //     .filter(|f| f.factor_source_id == factor_source_id)
        //     .map(|f| f.derivation_path())
        //     .map(|p| assert_matches.matches(&p))
        //     .map(|p| p.index)
        //     .max()
        todo!()
    }
}
impl HighestDerivationPathIndex for SecuredEntityControl {
    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        // self.matrix
        // .highest_derivation_path_index(factor_source_id, assert_matches)
        todo!()
    }
}

// pub trait IsSecurifiedEntity:
//     Hash + Eq + Clone + IsNetworkAware + HasEntityKind + TryFrom<AccountOrPersona>
// {
//     type BaseEntity: IsEntity + std::hash::Hash + Eq;

//     fn securified_entity_control(&self) -> SecuredEntityControl;

//     fn new(
//         name: impl Into<DisplayName>,
//         address: <Self::BaseEntity as IsBaseEntity>::Address,
//         securified_entity_control: SecuredEntityControl,
//     ) -> Self;

//     fn highest_derivation_path_index(
//         &self,
//         factor_source_id: FactorSourceIDFromHash,
//         assert_matches: AssertMatches,
//     ) -> Option<HDPathComponent> {
//         self.securified_entity_control()
//             .highest_derivation_path_index(factor_source_id, assert_matches)
//     }
// }

pub trait IsSecurifiedEntity: Hash + Eq + Clone + IsNetworkAware {
    type BaseEntity: IsBaseEntity + std::hash::Hash + Eq;

    fn securified_entity_control(&self) -> SecuredEntityControl;

    // fn new(
    //     name: impl Into<DisplayName>,
    //     address: <Self::BaseEntity as IsBaseEntity>::Address,
    //     securified_entity_control: SecuredEntityControl,
    // ) -> Self;

    fn highest_derivation_path_index(
        &self,
        factor_source_id: FactorSourceIDFromHash,
        assert_matches: AssertMatches,
    ) -> Option<HDPathComponent> {
        self.securified_entity_control()
            .highest_derivation_path_index(factor_source_id, assert_matches)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AbstractSecurifiedEntity<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> {
    __hidden: HiddenConstructor,
    pub entity: E,
    pub securified_entity_control: SecuredEntityControl,
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> IsNetworkAware
    for AbstractSecurifiedEntity<E>
{
    fn network_id(&self) -> NetworkID {
        self.entity.network_id()
    }
}

// impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> TryFrom<AccountOrPersona>
//     for AbstractSecurifiedEntity<E>
// {
//     type Error = CommonError;

//     fn try_from(value: AccountOrPersona) -> Result<Self> {
//         if TypeId::of::<E>() == TypeId::of::<AccountOrPersona>() {
//             let self_ =
//                 AbstractSecurifiedEntity::<AccountOrPersona>::new(value)?;
//             return Ok(self_ as Self);
//         }
//         match (E::entity_kind(), value) {
//             (CAP26EntityKind::Account, AccountOrPersona::AccountEntity(a)) => {
//                 Self::new(a)
//             }
//             (CAP26EntityKind::Identity, AccountOrPersona::PersonaEntity(p)) => {
//                 Self::new(p)
//             }
//             _ => Err(CommonError::Unknown),
//         }
//     }
// }

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> IsSecurifiedEntity
    for AbstractSecurifiedEntity<E>
{
    fn securified_entity_control(&self) -> SecuredEntityControl {
        self.securified_entity_control.clone()
    }
    type BaseEntity = E;
}
impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone>
    AbstractSecurifiedEntity<E>
{
    pub fn new(entity: E) -> Result<Self> {
        let securified_entity_control = entity.try_get_secured_control()?;
        Ok(Self {
            __hidden: HiddenConstructor,
            entity,
            securified_entity_control,
        })
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        Into::<AddressOfAccountOrPersona>::into(self.entity.address())
    }

    pub fn veci(&self) -> Option<VirtualEntityCreatingInstance> {
        // self.veci.clone()
        todo!()
    }
}

pub type AnySecurifiedEntity = AbstractSecurifiedEntity<AccountOrPersona>;
pub type SecurifiedAccount = AbstractSecurifiedEntity<Account>;
pub type SecurifiedPersona = AbstractSecurifiedEntity<Persona>;
impl SecurifiedAccount {
    pub fn erase_to_any(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(AccountOrPersona::from(self.entity.clone()))
            .unwrap()
    }
}
impl SecurifiedPersona {
    pub fn erase_to_any(&self) -> AnySecurifiedEntity {
        AnySecurifiedEntity::new(AccountOrPersona::from(self.entity.clone()))
            .unwrap()
    }
}
impl HasEntityKind for SecurifiedAccount {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}
impl HasEntityKind for SecurifiedPersona {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}
impl TryFrom<AccountOrPersona> for AnySecurifiedEntity {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Self::new(value)
    }
}
impl TryFrom<AccountOrPersona> for SecurifiedAccount {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Account::try_from(value).and_then(Self::new)
    }
}
impl TryFrom<AccountOrPersona> for SecurifiedPersona {
    type Error = CommonError;

    fn try_from(value: AccountOrPersona) -> Result<Self> {
        Persona::try_from(value).and_then(Self::new)
    }
}
