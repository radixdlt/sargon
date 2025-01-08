/// Whether to continue collecting signatures or finish early.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignaturesCollectingContinuation {
    /// It is meaningless to continue collecting signatures, either since either
    /// all transactions are valid, and the collector is configured to finish early
    /// in that case, or some transaction is invalid and the collector is configured
    /// finish early in that case.
    FinishEarly,

    /// We should continue collecting signatures, either since the collector is
    /// configured to not finish early, even though we can, or since we cannot
    /// finish early since not enough factor sources have been signed with.
    Continue,
}
