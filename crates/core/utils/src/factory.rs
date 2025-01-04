use crate::prelude::*;

pub fn iso8601(dt: &Timestamp) -> String {
    let (h, m, s) = dt.as_hms();
    format!("{} {:02}:{:02}:{:02}", date(dt), h, m, s)
}

pub fn date(dt: &Timestamp) -> String {
    dt.date().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::prelude::*;

    #[test]
    fn id_unique() {
        let n = 100;
        let set = (0..n).map(|_| Uuid::new_v4()).collect::<BTreeSet<Uuid>>();
        assert_eq!(set.len(), n);
    }

    #[test]
    fn date_str() {
        assert_eq!(date(&Timestamp::UNIX_EPOCH), "1970-01-01");
        assert_eq!(iso8601(&Timestamp::UNIX_EPOCH), "1970-01-01 00:00:00");
    }
}
