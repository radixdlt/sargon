pub fn main() {
    uniffi::generate_scaffolding("src/radix_connect.udl")
        .expect("Should be able to build.");
}
