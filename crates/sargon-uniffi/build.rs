use cargo_toml::{Dependency, Manifest};
use std::env;
use std::path::Path;

pub fn main() {
    uniffi::generate_scaffolding("src/sargon.udl")
        .expect("Should be able to build.");
}
