use crate::prelude::*;
use serde::{Deserializer, Serializer};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// A struct containing the necessary information to display the summary of an interaction in hosts.
/// It will be implemented on a second iteration with models similar to what we use on Transaction History.
pub struct InteractionQueueItemSummary {}
