use crate::prelude::*;

pub trait TestVector {
    fn test_vectors() -> Vec<Self> where Self: Sized;
}

impl TestVector for RequestedNumberQuantifier {
    fn test_vectors() -> Vec<Self> {
        vec![
            RequestedNumberQuantifier::Exactly,
            RequestedNumberQuantifier::AtLeast,
        ]
    }
}

impl TestVector for bool {
    fn test_vectors() -> Vec<Self> {
        vec![true, false]
    }
}

impl<T: TestVector> TestVector for Option<T> {
    fn test_vectors() -> Vec<Self> {
        let mut vectors = vec![None];
        let inner_vectors = T::test_vectors();
        vectors.extend(inner_vectors.into_iter().map(Some));
        vectors
    }
}