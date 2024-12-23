use crate::prelude::*;

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

pub fn id() -> Uuid {
    Uuid::new_v4()
}


pub fn iso8601(dt: &Timestamp) -> String {
    let (h, m, s) = dt.as_hms();
    format!("{} {:02}:{:02}:{:02}", date(dt), h, m, s)
}

pub fn date(dt: &Timestamp) -> String {
    dt.date().to_string()
}

impl HasSampleValues for Uuid {
    fn sample() -> Self {
        Self::from_bytes([0xff; 16])
    }

    fn sample_other() -> Self {
        Self::from_bytes([0xde; 16])
    }
}
impl HasSampleValues for Timestamp {
    fn sample() -> Self {
        Self::parse("2023-09-11T16:05:56Z").unwrap()
    }

    fn sample_other() -> Self {
        Self::parse("2023-12-24T17:13:56.123Z").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn id_unique() {
        let n = 100;
        let set = (0..n).map(|_| id()).collect::<BTreeSet<Uuid>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn date_str() {
        assert_eq!(date(&Timestamp::UNIX_EPOCH), "1970-01-01");
        assert_eq!(iso8601(&Timestamp::UNIX_EPOCH), "1970-01-01 00:00:00");
    }
}
