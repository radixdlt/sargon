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

impl<T> HasSampleValues for Vec<T>
where
    T: HasSampleValues,
{
    fn sample() -> Self {
        vec![T::sample()]
    }

    fn sample_other() -> Self {
        vec![T::sample_other()]
    }
}

impl<T> HasSampleValues for Option<T>
where
    T: HasSampleValues,
{
    fn sample() -> Self {
        Some(T::sample())
    }

    fn sample_other() -> Self {
        Some(T::sample_other())
    }
}

impl<Key, Value> HasSampleValues for HashMap<Key, Value>
where
    Key: HasSampleValues + Eq + std::hash::Hash,
    Value: HasSampleValues,
{
    fn sample() -> Self {
        let mut map = HashMap::new();
        map.insert(Key::sample(), Value::sample());
        map
    }

    fn sample_other() -> Self {
        let mut map = HashMap::new();
        map.insert(Key::sample_other(), Value::sample_other());
        map
    }
}
