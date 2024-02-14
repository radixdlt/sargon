pub fn main() {
    uniffi::generate_scaffolding("src/sargon.udl")
        .expect("Build script panics can be ignored");
}
