pub trait HasManySampleValues: Sized {
    #[allow(dead_code)]
    fn sample_all() -> Vec<Self>;
}
