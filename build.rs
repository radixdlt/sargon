use cargo_toml::{Dependency, Manifest};
use std::env;
use std::path::Path;

pub fn main() {
    let manifest_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = Manifest::from_path(manifest_path).expect("Can't panic");

    let set_dep_env = |key: &str| {
        let dependency = manifest.dependencies.get(key).expect("Can't panic");
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
