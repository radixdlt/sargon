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

impl HasSampleValues for DependencyInformation {
    fn sample() -> Self {
        let dep = "alamofire-rs";
        std::env::set_var(dep, format!("branch = {}", "develop"));
        let val = Self::of(dep);
        std::env::remove_var(dep);
        val
    }

    fn sample_other() -> Self {
        let dep = "the-composable-arc-rs";
        std::env::set_var(dep, format!("tag = {}", "2.3.7"));
        let val = Self::of(dep);
        std::env::remove_var(dep);
        val
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = DependencyInformation;

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
    fn value_of_sample() {
        assert_eq!(
            SUT::sample(),
            SUT::Branch {
                value: "develop".to_owned()
            }
        );
    }

    #[test]
    fn value_of_sample_other() {
        assert_eq!(
            SUT::sample_other(),
            SUT::Tag {
                value: "2.3.7".to_owned()
            }
        );
    }
}
