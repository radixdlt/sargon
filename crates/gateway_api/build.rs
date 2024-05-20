pub fn main() {
    uniffi::generate_scaffolding("src/gateway.udl").unwrap()
}
