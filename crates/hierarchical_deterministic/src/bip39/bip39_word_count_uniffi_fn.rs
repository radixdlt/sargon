use crate::prelude::*;

#[uniffi::export]
pub fn bip39_word_count_all() -> Vec<BIP39WordCount> {
    BIP39WordCount::all()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bip39_word_count_all() {
        assert_eq!(bip39_word_count_all().len(), 5);
    }
}
