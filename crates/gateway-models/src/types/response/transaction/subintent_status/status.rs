use crate::prelude::*;

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum SubintentStatus {
    /// The subintent status isn't final yet. This can either be because it wasn't committed yet, or
    /// because it was committed as a failure N times. Unlike a transaction intent, a subintent can
    /// be committed as a failure multiple times before succeeding.
    Unknown,

    /// The subintent was committed successfully, and its status won't ever change.
    CommittedSuccess,
}
