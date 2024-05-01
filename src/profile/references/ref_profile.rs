use crate::prelude::*;

decl_ref!(
    /// A reference type holding a "inner" value of type `Profile`, used
    /// primarily for fast JSON serialization and deserialization across the
    /// UniFFI boundary, 200x speed ups have been measured for huge Profiles
    /// (1000 accounts).
    ///
    /// The inner value is read out by calling `take` which will **consume** the
    /// value. Calling `take` on the FFI side twice will throw an error.
    /// (not possible to call it twice in Rust, since `take` takes `Arc<Self>`
    /// and consumes it.)
    Profile
);
