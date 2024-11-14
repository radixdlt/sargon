use crate::prelude::*;

pub trait IsEntity {
    /// An order set of `EntityFlag`s used to describe certain Off-ledger
    /// user state about Accounts or Personas, such as if an entity is
    /// marked as hidden or not.
    fn flags(&self) -> EntityFlags;

    fn is_hidden(&self) -> bool {
        self.flags()
            .into_iter()
            .contains(&EntityFlag::DeletedByUser)
    }

    fn is_tombstoned(&self) -> bool {
        self.flags()
            .into_iter()
            .contains(&EntityFlag::TombstonedByUser)
    }
}
