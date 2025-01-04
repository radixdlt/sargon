/// A macro that generates a XYZVersion type, which is a typed version of `u64`.
#[macro_export]
macro_rules! decl_version_type {
    ($name:ident) => {
        paste::paste! {
            #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
            #[serde(transparent)]
            pub struct [<$name Version>](pub u64);

            impl HasSampleValues for [<$name Version>] {
                fn sample() -> Self {
                    Self(1)
                }

                fn sample_other() -> Self {
                    Self(2)
                }
            }

            impl From<u64> for [<$name Version>] {
                fn from(value: u64) -> Self {
                    Self(value)
                }
            }

            impl std::ops::Deref for [<$name Version>] {
                type Target = u64;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    decl_version_type!(Example);

    use crate::prelude::*;

    #[test]
    fn equality() {
        assert_eq!(ExampleVersion::sample(), ExampleVersion::sample());
        assert_eq!(
            ExampleVersion::sample_other(),
            ExampleVersion::sample_other()
        );
    }

    #[test]
    fn inequality() {
        assert_ne!(ExampleVersion::sample(), ExampleVersion::sample_other());
    }

    #[test]
    fn modification() {
        let mut sut: ExampleVersion = 5.into();
        assert_eq!(*sut, 5);
        assert_eq!(sut.0, 5);

        sut.0 = 10;
        assert_eq!(*sut, 10);
        assert_eq!(sut.0, 10);
    }

    #[test]
    fn json_transparent() {
        #[derive(Deserialize, Serialize, PartialEq, Debug)]
        struct Test {
            name: String,
            version: ExampleVersion,
        }

        let sut = Test {
            name: "test".to_string(),
            version: ExampleVersion(25),
        };
        assert_eq_after_json_roundtrip(
            &sut,
            r#"
            {
                "name": "test",
                "version": 25
            }
            "#,
        );
    }
}
