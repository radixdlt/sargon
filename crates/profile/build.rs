use std::path::Path;

pub fn main() {
    // Paths for reading fixtures used by tests
    let fixtures_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
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

    let fixtures_models_path = fixtures_path.join("models");
    println!(
        "cargo:rustc-env=FIXTURES_MODELS={}/",
        fixtures_models_path.display()
    );

    uniffi::generate_scaffolding("src/profile.udl").unwrap()
}
