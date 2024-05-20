use crate::prelude::*;

#[derive(Default)]
pub struct BucketFactory {
    next_id: std::cell::Cell<u64>,
}
impl BucketFactory {
    pub fn next(&self) -> Bucket {
        let next = self.next_id.get();
        let bucket = Bucket {
            name: format!("bucket_{}", next),
        };
        self.next_id.set(next + 1);
        bucket
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = BucketFactory;

    #[test]
    fn bucket_factory() {
        let sut = SUT::default();
        assert_eq!(sut.next().name, "bucket_0");
        assert_eq!(sut.next().name, "bucket_1");
        assert_eq!(sut.next().name, "bucket_2");
    }
}
