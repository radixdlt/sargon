pub fn main() {
    uniffi::generate_scaffolding("src/sargoncommon.udl")
        .expect("Should be able to build.");
}
