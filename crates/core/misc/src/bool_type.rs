use crate::prelude::*;

#[macro_export]
macro_rules! decl_bool_type {
    ($name:ident, $default_value:expr) => {
        #[derive(
            Serialize,
            Deserialize,
            Debug,
            Copy,
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

        impl std::ops::Deref for $name {
            type Target = bool;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                $name(value)
            }
        }

        impl From<$name> for bool {
            fn from(value: $name) -> bool {
                value.0
            }
        }
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
        assert!(*ExampleTrue::default());

        assert!(!*ExampleFalse::default());
    }

    #[test]
    fn debug() {
        let str = "ExampleTrue(true)";
        let sut = ExampleTrue::sample();
        assert_eq!(format!("{:?}", sut), str);
    }

    #[test]
    fn modification() {
        let mut example_true = ExampleTrue::default();
        assert!(*example_true);
        example_true.0 = false;
        assert!(!*example_true);

        let mut example_false = ExampleFalse::default();
        assert!(!*example_false);
        example_false.0 = true;
        assert!(*example_false);
    }

    #[test]
    fn from_into() {
        let value = true;
        assert!(ExampleTrue::from(value).0);

        let value = false;
        assert!(!ExampleTrue::from(value).0);
    }
}
