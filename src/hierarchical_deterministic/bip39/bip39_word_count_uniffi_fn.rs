use crate::prelude::*;

#[uniffi::export]
pub fn bip39_word_count_all() -> Vec<BIP39WordCount> {
    BIP39WordCount::all()
}
