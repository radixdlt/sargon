pub fn main() {
    uniffi::generate_scaffolding("src/profile.udl").expect("Build script panics can be ignored");
}
