use crate::prelude::*;
use sargon::MessageContents as InternalMessageContents;

/// We explicitly mark content as either String or Bytes - this distinguishes (along with the mime type)
/// whether the message is intended to be displayable as text, or not.
///
/// This data model ensures that messages intended to be displayable as text are valid unicode strings.
#[derive(Clone, PartialEq, Eq, Hash, InternalConversion, uniffi::Enum)]
pub enum MessageContents {
    StringMessage { string: String },
    BinaryMessage { bag_of_bytes: BagOfBytes },
}