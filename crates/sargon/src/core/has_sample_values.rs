use crate::prelude::*;

pub trait HasSampleValues {
    fn sample() -> Self;
    fn sample_other() -> Self;
}


impl HasSampleValues for String {
    fn sample() -> Self {
        "sample".to_string()
    }

    fn sample_other() -> Self {
        "sample_other".to_string()
    }
}

impl HasSampleValues for bool {
    fn sample() -> Self {
        true
    }

    fn sample_other() -> Self {
        false
    }
}

impl HasSampleValues for u64 {
    fn sample() -> Self {
        42
    }

    fn sample_other() -> Self {
        43
    }
}

impl HasSampleValues for u32 {
    fn sample() -> Self {
        42
    }

    fn sample_other() -> Self {
        43
    }
}

impl HasSampleValues for u8 {
    fn sample() -> Self {
        42
    }

    fn sample_other() -> Self {
        43
    }
}

impl HasSampleValues for u16 {
    fn sample() -> Self {
        42
    }

    fn sample_other() -> Self {
        43
    }
}

impl HasSampleValues for Url {
    fn sample() -> Self {
        "http://example.com".parse().unwrap()
    }

    fn sample_other() -> Self {
        "http://example.org".parse().unwrap()
    }
}