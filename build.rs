use cargo_toml::{Dependency, Manifest};
use std::env;
use std::path::Path;

pub fn main() {
    let manifest_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");

    // Paths for reading fixtures used by tests
    let fixtures_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    println!("cargo:rustc-env=FIXTURES={}/", fixtures_path.display());
    let fixtures_transaction_path = fixtures_path.join("transaction");
    println!(
        "cargo:rustc-env=FIXTURES_TX={}/",
        fixtures_transaction_path.display()
    );
    let fixtures_vector_path = fixtures_path.join("vector");
    println!(
        "cargo:rustc-env=FIXTURES_VECTOR={}/",
        fixtures_vector_path.display()
    );

    let manifest = Manifest::from_path(manifest_path).expect("Can't panic");
    let dependencies = manifest.dependencies;
    let set_dep_env = |key: &str| {
        let dependency = dependencies.get(key).expect("Can't panic");
        let env_var_value = match dependency {
            Dependency::Simple(version) => format!("version={version}"),
            Dependency::Inherited(_) => {
                panic!("Inherited dependency is not supported")
            }
            Dependency::Detailed(detailed) => {
                if let Some(ref version) = detailed.version {
                    format!("version={version}")
                } else if let Some(ref branch) = detailed.branch {
                    format!("branch={branch}")
                } else if let Some(ref tag) = detailed.tag {
                    format!("tag={tag}")
                } else if let Some(ref rev) = detailed.rev {
                    format!("rev={rev}")
                } else {
                    panic!("Can't find version of {key} dependency")
                }
            }
        };
        let env_var =
            format!("{}_DEPENDENCY={}", key.to_uppercase(), env_var_value);
        println!("cargo:rustc-env={}", env_var);
    };

    set_dep_env("radix-engine");
    set_dep_env("radix-engine-toolkit");

    uniffi::generate_scaffolding("src/sargon.udl")
        .expect("Build script panics can be ignored");
}
