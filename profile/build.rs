pub fn main() {
    uniffi::generate_scaffolding("src/profileFFI.udl").expect("Build script panics can be ignored");
}
