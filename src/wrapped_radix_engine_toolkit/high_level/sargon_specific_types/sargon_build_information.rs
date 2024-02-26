use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SargonBuildInformation {
    pub sargon_version: String,
    pub dependencies: SargonDependencies,
}

impl SargonBuildInformation {
    pub fn get() -> Self {
        let sargon_version = env!("CARGO_PKG_VERSION").into();

        Self {
            sargon_version,
            dependencies: SargonDependencies::get(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, uniffi::Record)]
pub struct SargonDependencies {
    pub radix_engine_toolkit: DependencyInformation,
    pub scrypto_radix_engine: DependencyInformation,
}
impl SargonDependencies {
    pub fn get() -> Self {
        Self {
            radix_engine_toolkit: DependencyInformation::of(
                "RADIX-ENGINE-TOOLKIT-DEPENDENCY",
            ),
            scrypto_radix_engine: DependencyInformation::of(
                "RADIX-ENGINE-DEPENDENCY",
            ),
        }
    }
}

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
    fn of(name: &str) -> Self {
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
