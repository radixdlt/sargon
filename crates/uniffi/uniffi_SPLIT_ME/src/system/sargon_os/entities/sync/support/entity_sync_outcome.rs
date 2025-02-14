use crate::prelude::*;
use sargon::EntitySyncOutcome as InternalEntitySyncOutcome;

/// The report that gathers the different actions performed on profile after sync completes.
#[derive(Clone, Debug, InternalConversion, uniffi::Record)]
pub struct EntitySyncOutcome {
    pub actions_performed: Vec<EntitySyncActionPerformed>,
}
