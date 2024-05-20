pub fn main() {
    uniffi::generate_scaffolding("src/transaction.udl").unwrap()
}
