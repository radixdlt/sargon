/// 5 minutes
pub const EPOCH_DURATION_IN_SECONDS: u64 = 300;

/// As per the transaction validation configuration, epoch diff should be less than 1 month.
pub const MAX_EPOCH_DIFF: u64 = 30 * 24 * 60 * 60 / EPOCH_DURATION_IN_SECONDS;

/// 1 epoch for the fact that it's min_inclusive and max_exclusive;
/// 1 more for the fact that we might be very close to the end of the epoch already
pub const MIN_EPOCH_DIFF: u64 = 2;

/// Amount of items that should be requested on each page.
pub const GATEWAY_PAGE_REQUEST_LIMIT: u64 = 25;

/// Max amount of transfers that can be included in one single transaction.
pub const MAX_TRANSFERS_PER_TRANSACTION: u64 = 50;

/// Max amount of non fungibles to be queried in one request.
pub const GATEWAY_CHUNK_NON_FUNGIBLES: u64 = 100;

/// Max amount of addresses to be queried in one request to `/state/entity/details/`.
pub const GATEWAY_ENTITY_DETAILS_CHUNK_ADDRESSES: usize = 20;

/// Minimum XRD balance required for a fee payer to execute an account deletion transaction.
/// Includes a safety margin above the estimated 3 XRD fee for maximum resource transfer.
pub const MIN_REQUIRED_XRD_FOR_ACCOUNT_DELETION: f64 = 4.0;

/// The delay increment among polling requests is set to 1 second.
/// This means that there will be a 2s delay after first call, a 3s delay after second call, 4s after third and so on.
pub const POLLING_DELAY_INCREMENT_IN_SECONDS: u64 = 1;

/// Number of minutes per day.
pub const MINUTES_PER_DAY: u32 = 24 * 60;

/// Number of days per week.
pub const DAYS_PER_WEEK: u16 = 7;

/// Max amount of role requirements (resource addresses or non-fungible ids) to be queried in one request.
pub const GATEWAY_CHUNK_ENTITIES_BY_ROLE_REQUIREMENT_LOOKUP: u64 = 50;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_of_minutes_per_day() {
        assert_eq!(MINUTES_PER_DAY, 1440);
    }
}
