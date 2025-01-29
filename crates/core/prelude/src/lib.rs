#[macro_export]
macro_rules! fixture_in {
    ($path: expr, $file: expr) => {
        include_str!(concat!(env!("CARGO_WORKSPACE_DIR"), $path, "/", $file))
    };
}

#[macro_export]
macro_rules! fixture_tx_file {
    ($file: expr) => {
        $crate::fixture_in!(env!("FIXTURES_TX"), $file)
    };
}

#[macro_export]
macro_rules! fixture_rtm {
    ($file: expr) => {
        $crate::fixture_tx_file!(concat!($file, ".rtm"))
    };
}

#[macro_export]
macro_rules! fixture_tx {
    ($file: expr) => {
        $crate::fixture_tx_file!(concat!($file, ".json"))
    };
}

#[macro_export]
macro_rules! fixture_vector {
    ($file: expr) => {
        $crate::fixture_in!(env!("FIXTURES_VECTOR"), concat!($file, ".json"))
    };
}

#[macro_export]
macro_rules! fixture_model {
    ($file: expr) => {
        $crate::fixture_in!(env!("FIXTURES_MODELS"), concat!($file, ".json"))
    };
}

#[macro_export]
macro_rules! fixture_gw_model {
    ($file: expr) => {
        $crate::fixture_in!(env!("FIXTURES_MODELS_GW"), concat!($file, ".json"))
    };
}

#[macro_export]
macro_rules! fixture_profiles {
    ($file: expr) => {
        $crate::fixture_in!(
            env!("FIXTURES_MODELS_PROFILES"),
            concat!($file, ".json")
        )
    };
}

#[macro_export]
macro_rules! fixture_profile_model {
    ($file: expr) => {
        $crate::fixture_in!(
            env!("FIXTURES_MODELS_PROFILE"),
            concat!($file, ".json")
        )
    };
}

#[macro_export]
macro_rules! fixture_interaction {
    ($file: expr) => {
        $crate::fixture_in!(
            env!("FIXTURES_MODELS_INTERACTION"),
            concat!($file, ".json")
        )
    };
}

pub mod prelude {
    pub use std::collections::HashSet;
    pub use std::str::FromStr;
    pub use std::sync::{Arc, RwLock};
}

pub use prelude::*;
