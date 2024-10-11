use crate::prelude::*;
use sargon::MessageContents as InternalMessageContents;

/// We explicitly mark content as either String or Bytes - this distinguishes (along with the mime type)
/// whether the message is intended to be displayable as text, or not.
///
/// This data model ensures that messages intended to be displayable as text are valid unicode strings.
#[derive(Clone, PartialEq, Eq, Hash, EnumAsInner, uniffi::Enum)]
pub enum MessageContents {
    StringMessage { string: String },
    BinaryMessage { bag_of_bytes: BagOfBytes },
}

impl From<InternalMessageContents> for MessageContents {
    fn from(value: InternalMessageContents) -> Self {
        match value {
            InternalMessageContents::StringMessage { string } => {
                MessageContents::StringMessage { string }
            }
            InternalMessageContents::BinaryMessage { bag_of_bytes } => {
                MessageContents::BinaryMessage {
                    bag_of_bytes: bag_of_bytes.into(),
                }
            }
        }
    }
}

impl Into<InternalMessageContents> for MessageContents {
    fn into(self) -> InternalMessageContents {
        match self {
            MessageContents::StringMessage { string } => {
                InternalMessageContents::StringMessage { string }
            }
            MessageContents::BinaryMessage { bag_of_bytes } => {
                InternalMessageContents::BinaryMessage {
                    bag_of_bytes: bag_of_bytes.into(),
                }
            }
        }
    }
}
