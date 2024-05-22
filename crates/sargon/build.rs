pub fn main() {
    uniffi::generate_scaffolding("src/sargon.udl")
        .expect("Should be able to build.");
}
