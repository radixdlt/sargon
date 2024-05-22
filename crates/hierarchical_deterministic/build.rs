pub fn main() {
    uniffi::generate_scaffolding("src/hd.udl")
        .expect("Should be able to build.");
}
