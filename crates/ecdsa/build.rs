pub fn main() {
    uniffi::generate_scaffolding("src/ecdsa.udl").unwrap()
}
