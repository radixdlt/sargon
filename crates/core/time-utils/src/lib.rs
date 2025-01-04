use iso8601_timestamp::Timestamp;

/// A JSON "stable" timestamp, that is to say, this has already been JSON
/// roundtripped, ensuring the same value will be decoded once encoded, this is
/// a bit hacky solution to the fact that `07:18:08.284647000Z` when encoded
/// and then decoded become `07:18:08.284000000Z` resulting in problems for
/// equality checks.
pub fn now() -> Timestamp {
    let t = Timestamp::now_utc();
    let json = serde_json::to_vec(&t).unwrap();
    serde_json::from_slice(json.as_slice()).unwrap()
}
