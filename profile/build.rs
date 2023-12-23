pub fn main() {
    uniffi::generate_scaffolding("src/radix_wallet_kit.udl")
        .expect("Build script panics can be ignored");
}
