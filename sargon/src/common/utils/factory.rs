use crate::prelude::*;

pub fn now() -> Timestamp {
    Timestamp::now_utc()
}

pub fn id() -> Uuid {
    Uuid::new_v4()
}

pub fn profile_id() -> ProfileID {
    ProfileID(id())
}

pub fn iso8601(dt: &Timestamp) -> String {
    let (h, m, s) = dt.as_hms();
    format!("{} {:02}:{:02}:{:02}", date(dt), h, m, s)
}

pub fn date(dt: &Timestamp) -> String {
    dt.date().to_string()
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn date_now() {
        let d0 = now();
        let mut d1 = now();
        for _ in 0..10 {
            d1 = now();
        }
        assert!(d1 > d0);
    }

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
