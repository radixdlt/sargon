pub fn main() {
    uniffi::generate_scaffolding("src/clients.udl")
        .expect("Should be able to build.");
}
