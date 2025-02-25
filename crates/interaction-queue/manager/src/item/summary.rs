use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
/// A struct containing the necessary information to display the summary of an interaction in hosts.
/// It will be implemented on a second iteration with models similar to what we use on Transaction History.
pub struct InteractionQueueItemSummary {}

impl InteractionQueueItemSummary {
    pub fn new() -> Self {
        Self {}
    }
}
