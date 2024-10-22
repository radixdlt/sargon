use crate::prelude::*;

#[uniffi::export]
pub fn new_message_v2_plaintext_sample() -> MessageV2 {
    MessageV2::sample()
}

#[uniffi::export]
pub fn new_message_v2_plaintext_sample_other() -> MessageV2 {
    MessageV2::sample_other()
}

#[uniffi::export]
pub fn new_message_v2_plaintext_string(string: String) -> MessageV2 {
    MessageV2::plain_text(string)
}

#[uniffi::export]
pub fn message_v2_as_plaintext(message: &MessageV2) -> Option<String> {
    message.as_plaintext()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inequality() {
        assert_ne!(
            new_message_v2_plaintext_sample(),
            new_message_v2_plaintext_sample_other()
        );
    }

    #[test]
    fn new_message_v2_plaintext_string_then_as_plaintext() {
        let text = "Hello Unit Test".to_owned();
        assert_eq!(
            message_v2_as_plaintext(&new_message_v2_plaintext_string(
                text.clone()
            )),
            Some(text)
        );
    }
}
