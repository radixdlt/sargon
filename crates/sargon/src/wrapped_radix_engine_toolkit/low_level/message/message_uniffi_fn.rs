use crate::prelude::*;

#[uniffi::export]
pub fn new_message_plaintext_sample() -> Message {
    Message::sample()
}

#[uniffi::export]
pub fn new_message_plaintext_sample_other() -> Message {
    Message::sample_other()
}

#[uniffi::export]
pub fn new_message_plaintext_string(string: String) -> Message {
    Message::plain_text(string)
}

#[uniffi::export]
pub fn message_as_plaintext(message: &Message) -> Option<String> {
    message.as_plaintext()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inequality() {
        assert_ne!(
            new_message_plaintext_sample(),
            new_message_plaintext_sample_other()
        );
    }

    #[test]
    fn new_message_plaintext_string_then_as_plaintext() {
        let text = "Hello Unit Test".to_owned();
        assert_eq!(
            message_as_plaintext(&new_message_plaintext_string(text.clone())),
            Some(text)
        );
    }
}
