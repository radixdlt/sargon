pub fn main() {
    uniffi::generate_scaffolding("src/drivers.udl")
        .expect("Should be able to build.");
}
