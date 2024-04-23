use crate::prelude::*;

decl_can_be_empty_identified_array_of!(
    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    AssetsExceptionList,
    AssetException
);

impl HasSampleValues for AssetsExceptionList {
    fn sample() -> Self {
        Self::from_iter([
            AssetException::sample(),
            AssetException::sample_other(),
        ])
    }

    fn sample_other() -> Self {
        Self::from_iter([AssetException::sample_other()])
    }
}
