use crate::prelude::*;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum::EnumString,
    strum::Display,
)]
#[strum(serialize_all = "UPPERCASE")]
pub enum NetworkMethod {
    Post,
    Get,
    Head,
}

impl HasSampleValues for NetworkMethod {
    fn sample() -> Self {
        NetworkMethod::Post
    }

    fn sample_other() -> Self {
        NetworkMethod::Get
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = NetworkMethod;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn str_roundtrip() {
        let test = |m: SUT, s: &str| {
            assert_eq!(SUT::from_str(s).unwrap(), m);
            assert_eq!(m.to_string(), s);
            assert_eq!(SUT::from_str(&m.to_string()).unwrap(), m);
        };
        test(SUT::Post, "POST");
        test(SUT::Get, "GET");
        test(SUT::Head, "HEAD");
    }
}
