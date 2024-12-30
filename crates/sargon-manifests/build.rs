use std::env;
use std::path::Path;

pub fn main() {
    // Paths for reading fixtures used by tests
    let fixtures_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures");
    println!("cargo:rustc-env=FIXTURES={}/", fixtures_path.display());

    let fixtures_tx_path = fixtures_path.join("transaction");
    println!(
        "cargo:rustc-env=FIXTURES_TX={}/",
        fixtures_tx_path.display()
    );
}