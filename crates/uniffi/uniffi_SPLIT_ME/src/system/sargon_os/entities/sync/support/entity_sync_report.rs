use crate::prelude::*;
use sargon::EntitySyncReport as InternalEntitySyncReport;

/// The report that gathers the different actions performed on profile after sync completes.
#[derive(Clone, Debug, InternalConversion, uniffi::Record)]
pub struct EntitySyncReport {
    pub actions_performed: Vec<EntitySyncActionPerformed>,
}
