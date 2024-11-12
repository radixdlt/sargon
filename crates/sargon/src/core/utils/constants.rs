// 5 minutes
pub const EPOCH_DURATION_IN_SECONDS: u64 = 300;

// As per the transaction validation configuration, epoch diff should be less than 1 month.
pub const MAX_EPOCH_DIFF: u64 = 30 * 24 * 60 * 60 / EPOCH_DURATION_IN_SECONDS;

// 1 epoch for the fact that it's min_inclusive and max_exclusive;
// 1 more for the fact that we might be very close to the end of the epoch already
pub const MIN_EPOCH_DIFF: u64 = 2;
