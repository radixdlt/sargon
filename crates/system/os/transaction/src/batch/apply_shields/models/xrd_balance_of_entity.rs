use crate::prelude::*;

pub struct XrdBalanceOfEntity<Entity: HasEntityAddress + Clone> {
    pub entity: Entity,
    pub balance: Decimal,
}

impl<Entity: HasEntityAddress + Clone> XrdBalanceOfEntity<Entity> {
    pub fn new(entity: impl Into<Entity>, balance: Decimal) -> Self {
        Self {
            entity: entity.into(),
            balance,
        }
    }
}
