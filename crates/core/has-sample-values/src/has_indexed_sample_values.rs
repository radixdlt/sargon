pub trait HasIndexedSampleValues: Sized {
    #[allow(dead_code)]
    fn sample_at(index: usize) -> Self;
}
