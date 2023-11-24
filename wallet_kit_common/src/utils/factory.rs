use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local()
}
pub fn id() -> Uuid {
    Uuid::new_v4()
}
fn date_to_string(dt: &NaiveDateTime, fmt: &str) -> String {
    dt.format(fmt).to_string()
}

pub fn iso8601(dt: &NaiveDateTime) -> String {
    date_to_string(dt, "%Y-%m-%d %H:%M:%S")
}

pub fn date(dt: &NaiveDateTime) -> String {
    date_to_string(dt, "%Y-%m-%d")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::utils::factory::iso8601;

    use super::{date, id, now};

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
        let set = (0..n).into_iter().map(|_| id()).collect::<BTreeSet<Uuid>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn date_str() {
        assert_eq!(date(&NaiveDateTime::UNIX_EPOCH), "1970-01-01");
        assert_eq!(iso8601(&NaiveDateTime::UNIX_EPOCH), "1970-01-01 00:00:00");
    }
}
