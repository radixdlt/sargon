use crate::prelude::*;

pub struct XrdBalanceOfEntity<Entity: HasEntityAddress + Clone> {
    pub entity: Entity,
    pub balance: Decimal,
}

impl<Entity: HasEntityAddress + Clone + Into<Account>>
    XrdBalanceOfEntity<Entity>
{
    pub fn into_account(self) -> XrdBalanceOfEntity<Account> {
        XrdBalanceOfEntity {
            entity: self.entity.into(),
            balance: self.balance,
        }
    }
}
impl<Entity: HasEntityAddress + Clone> XrdBalanceOfEntity<Entity> {
    pub fn new(entity: impl Into<Entity>, balance: Decimal) -> Self {
        Self {
            entity: entity.into(),
            balance,
        }
    }
}

impl<Entity: HasEntityAddress + Clone> HasEntityAddress
    for XrdBalanceOfEntity<Entity>
{
    fn address_erased(&self) -> AddressOfAccountOrPersona {
        self.entity.address_erased()
    }
}
