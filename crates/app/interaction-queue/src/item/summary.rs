use crate::prelude::*;
use serde::{Deserializer, Serializer};

#[derive(Debug, Clone, PartialEq)]
/// An enum describing the summary of an item in the interaction queue.
/// It will include the plain summary returned by RET when performing a review of the interaction. The downsides of this option:
/// - it isn't very optimal since it will require hosts to resolve the sections every time the queue is opened.
/// - we need to implement Serialize/Deserialize for the associated values.
pub enum InteractionQueueItemSummary {
    Execution(ExecutionSummary),

    Manifest(ManifestSummary),
}

impl Serialize for InteractionQueueItemSummary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        panic!("Cannot serialize InteractionQueueItemSummary");
    }
}

impl<'de> Deserialize<'de> for InteractionQueueItemSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        panic!("Cannot deserialize InteractionQueueItemSummary")
    }
}

/// Another summary version. It would include the necessary data used by hosts to display the UI. The downside of this option are two:
/// - we need to implement a lot of UI models in Sargon
/// - the data could be outdated after the interaction is resolved. For example, if an Account has its name
/// updated after the interaction is created, the queue would always show the initial name (since, e.g., the `withdrawals`)
/// would have information from the original account data.
struct InteractionQueueItemSummaryV2 {
    /*
    pub withdrawals: Option≤Accounts>,
    pub dapps_used: Option<InteractionReviewDappsUsed>,
    pub deposits: Option<Accounts>,
    // .. everything else that Hosts use to display the TransactionReview/PreAuthorizationReview
    */
}
