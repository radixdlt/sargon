pub fn main() {
    uniffi::generate_scaffolding("src/drivers.udl").unwrap()
}
