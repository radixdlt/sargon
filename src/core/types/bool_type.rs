use crate::prelude::*;

macro_rules! decl_bool_type {
    ($name:ident, $default_value:expr) => {
        #[derive(
            Serialize,
            Deserialize,
            Debug,
            PartialEq,
            Eq,
            Clone,
            Hash,
            derive_more::Display,
        )]
        #[serde(transparent)]
        pub struct $name(pub bool);

        impl Default for $name {
            fn default() -> Self {
                $name($default_value)
            }
        }

        impl HasSampleValues for $name {
            fn sample() -> Self {
                $name($default_value)
            }

            fn sample_other() -> Self {
                $name(!$default_value)
            }
        }

        uniffi::custom_newtype!($name, bool);
    };
}

pub(crate) use decl_bool_type;

#[cfg(test)]
mod tests {
    decl_bool_type!(ExampleTrue, true);
    decl_bool_type!(ExampleFalse, false);

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(ExampleTrue::sample(), ExampleTrue::sample());
        assert_eq!(ExampleTrue::sample_other(), ExampleTrue::sample_other());

        assert_eq!(ExampleFalse::sample(), ExampleFalse::sample());
        assert_eq!(ExampleFalse::sample_other(), ExampleFalse::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(ExampleTrue::sample(), ExampleTrue::sample_other());

        assert_ne!(ExampleFalse::sample(), ExampleFalse::sample_other());
    }

    #[test]
    fn default() {
        assert!(ExampleTrue::default().0);

        assert!(!ExampleFalse::default().0);
    }

    #[test]
    fn modification() {
        let mut example_true = ExampleTrue::default();
        assert!(example_true.0);
        example_true.0 = false;
        assert!(!example_true.0);

        let mut example_false = ExampleFalse::default();
        assert!(!example_false.0);
        example_false.0 = true;
        assert!(example_false.0);
    }
}
