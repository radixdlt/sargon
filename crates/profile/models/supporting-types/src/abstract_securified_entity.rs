use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
pub struct AbstractSecurifiedEntity<
    E: IsBaseEntity + std::hash::Hash + Eq + Clone,
> {
    #[allow(dead_code)]
    #[doc(hidden)]
    #[debug(skip)]
    __hidden: HiddenConstructor,
    pub entity: E,
    pub securified_entity_control: SecuredEntityControl,
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> Identifiable
    for AbstractSecurifiedEntity<E>
{
    type ID = AddressOfAccountOrPersona;
    fn id(&self) -> Self::ID {
        self.entity.address().into()
    }
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> HasEntityAddress
    for AbstractSecurifiedEntity<E>
{
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        self.entity.address_erased()
    }
}

impl<E: IsBaseEntity + std::hash::Hash + Eq + Clone> IsNetworkAware
    for AbstractSecurifiedEntity<E>
{
    fn network_id(&self) -> NetworkID {
        self.entity.network_id()
    }
}

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
    pub fn access_controller_address(&self) -> AccessControllerAddress {
        self.securified_entity_control.access_controller_address()
    }

    pub fn with_securified_entity_control(
        entity: E,
        securified_entity_control: SecuredEntityControl,
    ) -> Self {
        Self {
            __hidden: HiddenConstructor,
            entity,
            securified_entity_control,
        }
    }

    pub fn new(entity: E) -> Result<Self> {
        entity
            .try_get_secured_control()
            .map(|sec| Self::with_securified_entity_control(entity, sec))
    }

    pub fn address(&self) -> AddressOfAccountOrPersona {
        Into::<AddressOfAccountOrPersona>::into(self.entity.address())
    }

    pub fn current_authentication_signing_factor_instance(
        &self,
    ) -> HierarchicalDeterministicFactorInstance {
        self.securified_entity_control()
            .authentication_signing_factor_instance()
    }

    pub fn veci(&self) -> Option<VirtualEntityCreatingInstance> {
        self.securified_entity_control()
            .veci()
            .map(|fi| VirtualEntityCreatingInstance::new(fi, self.address()))
    }

    pub fn xrd_vault_address(&self) -> VaultAddress {
        self.securified_entity_control().xrd_vault_address()
    }
}
