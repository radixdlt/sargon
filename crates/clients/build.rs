pub fn main() {
    uniffi::generate_scaffolding("src/clients.udl").unwrap()
}
