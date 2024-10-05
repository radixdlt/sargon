use crate::prelude::*;

decl_identified_vec_of!(
    /// Denies or allows third-party deposits of specific assets by ignoring the `depositMode`
    AssetsExceptionList,
    AssetException
);
