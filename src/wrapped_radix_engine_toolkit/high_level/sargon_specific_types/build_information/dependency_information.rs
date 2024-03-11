use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, EnumAsInner, uniffi::Enum)]
pub enum DependencyInformation {
    // Crates.io
    Version { value: String },

    // Github
    Tag { value: String },
    Branch { value: String },
    Rev { value: String },
}

impl DependencyInformation {
    pub(crate) fn of(name: &str) -> Self {
        let version = std::env::var(name).expect("Valid env variable");

        let mut split = version.split('=');
        let identifier = split.next().expect("Should never fail").trim();
        let value = split.next().expect("Should never fail").trim();

        match identifier {
            "version" => Self::Version {
                value: value.into(),
            },
            "tag" => Self::Tag {
                value: value.into(),
            },
            "branch" => Self::Branch {
                value: value.into(),
            },
            "rev" => Self::Rev {
                value: value.into(),
            },
            _ => {
                unreachable!("Unknown identifier encountered: '{}'", identifier)
            }
        }
    }
}

// this is very unfortunate, but unit tests fail 1/20 due to `Valid env variable: NotPresent`
// probably due to multi thread, maybe we should ignore the tests below, or
// add some kind of lock, perhaps using `rusty_fork`
// see: https://crates.io/crates/rusty-fork
// see: https://github.com/numtide/treefmt/pull/253/files
fn set_get_unset(key: &str, value: &str) -> DependencyInformation {
    std::env::set_var(key, value);
    let val = DependencyInformation::of(key);
    std::env::remove_var(key);
    val
}

impl HasSampleValues for DependencyInformation {
    fn sample() -> Self {
        set_get_unset("alamofire-rs", "branch = develop")
    }

    fn sample_other() -> Self {
        set_get_unset("the-composable-arc-rs", "tag = 2.3.7")
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DependencyInformation;

    #[test]
    #[ignore]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    #[ignore]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn value_of_sample() {
        assert_eq!(
            SUT::sample(),
            SUT::Branch {
                value: "develop".to_owned()
            }
        );
    }

    #[test]
    #[ignore]
    fn value_of_sample_other() {
        assert_eq!(
            SUT::sample_other(),
            SUT::Tag {
                value: "2.3.7".to_owned()
            }
        );
    }
}
