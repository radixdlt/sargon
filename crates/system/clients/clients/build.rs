use std::env;
use std::path::Path;

pub fn main() {
    // Paths for reading fixtures used by tests
    let cargo = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = cargo.join("../../../../target");
    println!("cargo:rustc-env=TARGET_PATH={}/", target.display());
    let tmp = target.join("tmp");
    println!("cargo:rustc-env=TMP_PATH={}/", tmp.display());
}
