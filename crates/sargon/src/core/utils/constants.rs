// 5 minutes
pub const EPOCH_DURATION_IN_SECONDS: u64 = 300;

// As per the transaction validation configuration, epoch diff should be less than 1 month.
pub const MAX_EPOCH_DIFF: u64 = 30 * 24 * 60 * 60 / EPOCH_DURATION_IN_SECONDS;

// 1 epoch for the fact that it's min_inclusive and max_exclusive;
// 1 more for the fact that we might be very close to the end of the epoch already
pub const MIN_EPOCH_DIFF: u64 = 2;

// Amount of items that should be requested on each page.
pub const GATEWAY_PAGE_REQUEST_LIMIT: u64 = 25;

// Max amount of transfers that can be included in one single transaction.
pub const MAX_TRANSFERS_PER_TRANSACTION: u64 = 50;

// Max amount of non fungibles to be queried in one request.
pub const GATEWAY_CHUNK_NON_FUNGIBLES: u64 = 100;

// Max amount of addresses to be queried in one request.
pub const GATEWAY_CHUNK_ADDRESSES: u64 = 20;
