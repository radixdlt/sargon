use crate::prelude::*;
use sargon::EntitySyncActionPerformed as InternalEntitySyncActionPerformed;

/// The kinds of sync actions performed on entities in profile.
#[derive(Clone, Debug, InternalConversion, uniffi::Enum)]
pub enum EntitySyncActionPerformed {
    SomeEntitiesTombstoned,
    SomeEntitiesSecurified,
}
