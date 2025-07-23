use serde::Serializer;

use crate::prelude::*;

#[derive(
    Zeroize,
    Clone,
    /* NEVER COPY! We wanna require explicit copying */
    PartialEq,
    Eq,
    Hash,
    DeserializeFromStr,
    derive_more::Display,
    derive_more::Debug,
)]
#[display("{}", self.to_obfuscated_string())]
#[debug("{:?}", self.partially_obfuscated_string())]
pub struct Mnemonic {
    pub words: Vec<BIP39Word>,

    #[zeroize(skip)]
    pub word_count: BIP39WordCount,

    #[zeroize(skip)]
    pub language: BIP39Language,
}

impl Serialize for Mnemonic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.phrase().serialize(serializer)
    }
}
impl Mnemonic {
    pub fn partially_obfuscated_string(&self) -> String {
        format!(
            "{} ({}...{})",
            self.word_count,
            self.words.first().unwrap().word,
            self.words.last().unwrap().word
        )
    }
}

impl Mnemonic {
    pub fn to_obfuscated_string(&self) -> String {
        format!("Mnemonic in {} obfuscated.", self.language)
    }

    pub(crate) fn from_internal(internal: bip39::Mnemonic) -> Self {
        let language = internal.language();

        let words = internal
            .word_iter()
            .map(|w| BIP39Word::new(w, language.into()))
            .collect::<Result<Vec<BIP39Word>, CommonError>>()
            .expect("Crate bip39 generated words unknown to us.");

        let word_count = BIP39WordCount::from_count(internal.word_count())
            .expect(
            "Crate bip39 generated a BIP39 standard incompatible word count.",
        );

        drop(internal);

        Self {
            words,
            word_count,
            language: language.into(),
        }
    }

    fn internal(&self) -> bip39::Mnemonic {
        bip39::Mnemonic::from_str(&self.phrase()).unwrap()
    }

    pub fn phrase(&self) -> String {
        self.words.iter().map(|w| w.word.to_string()).join(" ")
    }

    pub fn from(phrase: &str, language: BIP39Language) -> Result<Self> {
        bip39::Mnemonic::parse_in(language.into(), phrase)
            .map_err(|_| CommonError::InvalidMnemonicPhrase)
            .map(Self::from_internal)
    }

    pub fn from_32bytes_entropy(entropy: Exactly32Bytes) -> Self {
        let internal: bip39::Mnemonic = bip39::Mnemonic::from_entropy(
            entropy.as_ref(),
        )
        .expect("Should always be able to create Mnemonic from 32 bytes");
        Self::from_internal(internal)
    }

    pub fn from_words(words: Vec<BIP39Word>) -> Result<Self> {
        if words.is_empty() {
            return Err(CommonError::InvalidMnemonicPhrase);
        }

        let language = words.first().unwrap().language;

        if words.iter().any(|w| w.language != language) {
            return Err(CommonError::InvalidMnemonicPhrase);
        }

        let phrase = words.iter().map(|w| w.word.to_string()).join(" ");
        Self::from_phrase(&phrase)
    }

    pub fn from_phrase(phrase: &str) -> Result<Self> {
        bip39::Mnemonic::from_str(phrase)
            .map_err(|_| CommonError::InvalidMnemonicPhrase)
            .map(Self::from_internal)
    }

    pub fn from_arculus_bytes(words: Vec<u8>) -> Result<Self> {
        let null_terminated_phrase =
            String::from_utf8(words).map_err(|_| {
                CommonError::ArculusCardInvalidNonUtf8MnemonicPhrase
            })?;
        let phrase = null_terminated_phrase.trim_end_matches('\0');
        Mnemonic::from_phrase(phrase)
    }

    pub fn to_entropy(&self) -> NonEmptyMax32Bytes {
        let entropy = self.internal().to_entropy();
        NonEmptyMax32Bytes::try_from(entropy)
            .expect("Entropy should never be empty and always max 32 bytes")
    }

    pub fn to_seed(&self, passphrase: &str) -> BIP39Seed {
        BIP39Seed::new(self.internal().to_seed(passphrase))
    }
}

impl FromStr for Mnemonic {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Mnemonic::from_phrase(s)
    }
}

impl HasSampleValues for Mnemonic {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::from_phrase("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate").expect("Valid mnemonic")
    }

    fn sample_other() -> Self {
        Self::from_phrase("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong")
            .expect("Valid mnemonic")
    }
}

#[cfg(test)]
pub(super) fn calculate_last_mnemonic_word_from_words(
    words: impl IntoIterator<Item = String>,
) -> Vec<Mnemonic> {
    let words = words
        .into_iter()
        .map(|x| BIP39Word::new(x.clone(), BIP39Language::English).unwrap())
        .collect_vec();
    let count = words.len();
    assert!(count == 11 || count == 23, "wrong word count");

    let mut mnemonics = Vec::new();
    for word in BIP39Language::English.wordlist().iter() {
        let mut all_words = words.clone();
        all_words.push(word.clone());
        match Mnemonic::from_words(all_words) {
            Ok(mnemonic) => mnemonics.push(mnemonic),
            Err(_) => continue,
        };
    }
    assert!(!mnemonics.is_empty(), "should find at least one!");
    mnemonics
}

#[cfg(test)]
pub(super) fn calculate_last_mnemonic_word_from_phrase(
    phrase: impl AsRef<str>,
) -> Vec<Mnemonic> {
    calculate_last_mnemonic_word_from_words(
        phrase.as_ref().split(' ').map(|x| x.to_owned()),
    )
}

impl Mnemonic {
    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * athlete
    /// * excite
    /// * jar
    /// * open
    /// * planet
    /// * swim
    /// * wrap
    pub fn sample_device() -> Self {
        Self::from_phrase("device phone sign source sample device sample device sample device sample device sample device sample device sample device phone sign source sample device swim")
        .expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * absurd
    /// * circle
    /// * expire
    /// * illness
    /// * paddle
    /// * rabbit
    /// * sniff
    /// * winner
    pub fn sample_device_other() -> Self {
        Self::from_phrase("device phone sign source sample other device sample other device sample other device sample other device sample other device sample other device other paddle").expect("Valid mnemonic")
    }

    pub fn sample_device_12_words() -> Self {
        Self::from_phrase("device twelve phone sign source sample device twelve sample device twelve original").expect("Valid mnemonic")
    }

    pub fn sample_device_12_words_other() -> Self {
        Self::from_phrase("device twelve phone sign source sample other device twelve sample other derive").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * assault
    /// * cactus
    /// * exile
    /// * legal
    /// * million
    /// * result
    /// * spy
    /// * version
    pub fn sample_ledger() -> Self {
        Self::from_phrase("pledge rely stick hard snow ice sign source sample pledge rely sample pledge rely sample pledge rely sample pledge rely sample stick sample cactus").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * basic
    /// * couch
    /// * elbow
    /// * head
    /// * ozone
    /// * popular
    /// * shaft
    /// * tunnel
    pub fn sample_ledger_other() -> Self {
        Self::from_phrase("pledge rely stick hard snow ice sign source sample other pledge rely sample other pledge rely sample other pledge rely stick sample other shaft").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * begin
    /// * cream
    /// * excite
    /// * insect
    /// * lizard
    /// * payment
    /// * state
    /// * wide
    pub fn sample_off_device() -> Self {
        Self::from_phrase("off device sign source sample off sample off sample off sample off sample off sample off sample off sample off sample off sample lizard").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * boss
    /// * coral
    /// * fruit
    /// * identify
    /// * local
    /// * pulse
    /// * talent
    /// * then
    pub fn sample_off_device_other() -> Self {
        Self::from_phrase("off device sign source sample other off sample other off sample other off sample other off sample other off device sample other off fruit").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * amateur
    /// * combine
    /// * fold
    /// * include
    /// * neutral
    /// * ritual
    /// * science
    /// * unveil
    pub fn sample_security_questions() -> Self {
        Self::from_phrase("security question sign source sample security question sample security question sample security question sample security question sample security question sample security question sample unveil").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * arrow
    /// * chief
    /// * ensure
    /// * impulse
    /// * loop
    /// * problem
    /// * sword
    /// * total
    pub fn sample_security_questions_other() -> Self {
        Self::from_phrase("security question sign source sample other security question sample other security question sample other security question sample other security question sample other question loop").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * bargain
    /// * discover
    /// * essay
    /// * govern
    /// * mix
    /// * power
    /// * silent
    /// * tobacco
    pub fn sample_arculus() -> Self {
        Self::from_phrase("arch card helmet sign source sample arch card sample arch card sample arch card sample arch card sample arch card sample arch card mix").expect("Valid mnemonic")
    }

    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * bomb
    /// * cream
    /// * dragon
    /// * gather
    /// * lock
    /// * prevent
    /// * soccer
    /// * update
    pub fn sample_arculus_other() -> Self {
        Self::from_phrase("arch card helmet sign source sample other arch card sample other arch card sample other arch card sample other arch card sample other lock").expect("Valid mnemonic")
    }

    #[allow(dead_code)]
    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * brass
    /// * crater
    /// * embrace
    /// * invest
    /// * music
    /// * project
    /// * uphold
    pub fn sample_password() -> Self {
        Self::from_phrase("pass phrase sign source sample pass phrase sign source sample pass phrase sign source sample pass phrase sign source sample pass phrase sample soon").expect("Valid mnemonic")
    }
    #[allow(dead_code)]
    /// Alternative valid mnemonics, with last (checksum) words changed only are:
    /// * animal
    /// * collect
    /// * dragon
    /// * gold
    /// * once
    /// * ripple
    /// * summer
    pub fn sample_password_other() -> Self {
        Self::from_phrase("pass phrase sign source sample other pass phrase sign source sample other pass phrase sign source sample other pass phrase source sample other usual").expect("Valid mnemonic")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Mnemonic;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn find_device_sample() {
        let s = "device phone sign source sample device sample device sample device sample device sample device sample device sample device phone sign source sample device";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_device()));
    }

    #[test]
    fn find_device_sample_other() {
        let s = "device phone sign source sample other device sample other device sample other device sample other device sample other device sample other device other";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_device_other()));
    }

    #[test]
    fn find_device_sample_12_words() {
        let s = "device twelve phone sign source sample device twelve sample device twelve";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_device_12_words()));
    }

    #[test]
    fn find_device_sample_12_words_other() {
        let s = "device twelve phone sign source sample other device twelve sample other";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics
            .iter()
            .contains(&SUT::sample_device_12_words_other()));
    }

    #[test]
    fn find_ledger_sample() {
        let s = "pledge rely stick hard snow ice sign source sample pledge rely sample pledge rely sample pledge rely sample pledge rely sample stick sample";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_ledger()));
    }

    #[test]
    fn find_ledger_sample_other() {
        let s = "pledge rely stick hard snow ice sign source sample other pledge rely sample other pledge rely sample other pledge rely stick sample other";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_ledger_other()));
    }

    #[test]
    fn find_off_device_sample() {
        let s = "off device sign source sample off sample off sample off sample off sample off sample off sample off sample off sample off sample";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_off_device()));
    }

    #[test]
    fn find_off_device_sample_other() {
        let s = "off device sign source sample other off sample other off sample other off sample other off sample other off device sample other off";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_off_device_other()));
    }

    #[test]
    fn find_security_questions_sample() {
        let s = "security question sign source sample security question sample security question sample security question sample security question sample security question sample security question sample";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_security_questions()));
    }

    #[test]
    fn find_security_questions_sample_other() {
        let s = "security question sign source sample other security question sample other security question sample other security question sample other security question sample other question";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics
            .iter()
            .contains(&SUT::sample_security_questions_other()));
    }

    #[test]
    fn find_arculus() {
        let s = "arch card helmet sign source sample arch card sample arch card sample arch card sample arch card sample arch card sample arch card";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_arculus()));
    }

    #[test]
    fn find_arculus_other() {
        let s = "arch card helmet sign source sample other arch card sample other arch card sample other arch card sample other arch card sample other";
        let mnemonics = calculate_last_mnemonic_word_from_phrase(s);
        assert!(mnemonics.iter().contains(&SUT::sample_arculus_other()));
    }

    #[test]
    fn debug() {
        let mnemonic = SUT::sample();
        assert_eq!(
            format!("{:?}", mnemonic),
            format!("{:?}", "24 words (bright...mandate)")
        );
    }

    #[test]
    fn display() {
        let mnemonic = SUT::sample();
        assert_eq!(format!("{}", mnemonic), "Mnemonic in English obfuscated.")
    }

    #[test]
    fn language() {
        let mnemonic: SUT =
            "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"
                .parse()
                .unwrap();
        assert_eq!(mnemonic.language, BIP39Language::English);
        mnemonic
            .words
            .into_iter()
            .for_each(|w| assert_eq!(w.language, BIP39Language::English));
    }

    #[test]
    fn word_count() {
        assert_eq!( SUT::from_phrase("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate").unwrap().word_count, BIP39WordCount::TwentyFour);
        assert_eq!(
            SUT::from_phrase(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            )
            .unwrap()
            .word_count,
            BIP39WordCount::Twelve
        );
    }

    #[test]
    fn words() {
        let mnemonic = SUT::sample();
        assert_eq!(mnemonic.words[0].word, "bright");
        assert_eq!(mnemonic.words[1].word, "club");
        assert_eq!(mnemonic.words[2].word, "bacon");
        assert_eq!(mnemonic.words[12].word, "humble");
        assert_eq!(mnemonic.words[22].word, "goose");
        assert_eq!(mnemonic.words[23].word, "mandate");
    }

    #[test]
    fn words_index() {
        let zoo: SUT = "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            .parse()
            .unwrap();
        assert_eq!(zoo.words[0].index.inner, 2047);
        assert_eq!(zoo.words[1].index.inner, 2047);
        assert_eq!(zoo.words[10].index.inner, 2047);
        assert_eq!(zoo.words[11].index.inner, 2037);

        let abandon: SUT = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
            .parse()
            .unwrap();
        assert_eq!(abandon.words[0].index.inner, 0);
        assert_eq!(abandon.words[1].index.inner, 0);
        assert_eq!(abandon.words[10].index.inner, 0);
        assert_eq!(abandon.words[11].index.inner, 3);
    }

    #[test]
    fn phrase_str_roundtrip() {
        let phrase = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate";
        let mnemonic = SUT::from_phrase(phrase).unwrap();
        assert_eq!(mnemonic.phrase(), phrase);
    }

    #[test]
    fn from_phrase_invalid() {
        assert_eq!(
            SUT::from_phrase(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon"
            ),
            Err(CommonError::InvalidMnemonicPhrase)
        );
    }

    #[test]
    fn from_phrase_language() {
        assert_eq!(
            SUT::from(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
                BIP39Language::English
            ),
            SUT::from_phrase(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            )
        );
    }

    #[test]
    fn from_wrong_phrase_language() {
        assert_eq!(
            SUT::from(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon",
                BIP39Language::English
            ),
            Err(CommonError::InvalidMnemonicPhrase)
        );
    }

    #[test]
    fn from_words() {
        assert_eq!(
            SUT::from_words(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
                    .split(' ')
                    .map(|w| BIP39Word::new(w, BIP39Language::English).unwrap())
                    .collect_vec()
            ),
            SUT::from_phrase(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"
            )
        );
    }

    #[test]
    fn from_words_empty_phrase() {
        assert_eq!(
            SUT::from_words(vec![]),
            Err(CommonError::InvalidMnemonicPhrase)
        );
    }

    #[test]
    fn from_words_wrong_phrase() {
        assert_eq!(
            SUT::from_words(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon"
                    .split(' ')
                    .map(|w| BIP39Word::new(w, BIP39Language::English).unwrap())
                    .collect_vec()
            ),
            Err(CommonError::InvalidMnemonicPhrase)
        );
    }

    #[test]
    fn from_arculus_bytes_bad_representation() {
        let non_utf8_bytes: Vec<u8> = vec![0x80, 0xFF, 0xC0, 0xC1];
        assert_eq!(
            SUT::from_arculus_bytes(non_utf8_bytes),
            Err(CommonError::ArculusCardInvalidNonUtf8MnemonicPhrase)
        )
    }

    #[test]
    fn from_words_wrong_words_count() {
        assert_eq!(
            SUT::from_words(
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo"
                    .split(' ')
                    .map(|w| BIP39Word::new(w, BIP39Language::English).unwrap())
                    .collect_vec()
            ),
            Err(CommonError::InvalidMnemonicPhrase)
        );
    }

    #[test]
    fn json_roundtrip_success() {
        let a: SUT = "bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"
            .parse()
            .unwrap();

        assert_json_value_eq_after_roundtrip(
            &a,
            json!("bright club bacon dinner achieve pull grid save ramp cereal blush woman humble limb repeat video sudden possible story mask neutral prize goose mandate"),
        );
        assert_json_roundtrip(&a);
        assert_json_value_ne_after_roundtrip(
            &a,
            json!("zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong"),
        );
    }

    #[test]
    fn json_fails() {
        assert_json_value_fails::<SUT>(json!("invalid"));
        assert_json_value_fails::<SUT>(json!(
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon"
        )); // not checksummed
        assert_json_value_fails::<SUT>(json!(
            "hej jag zoo zoo zoo zoo zoo zoo zoo zoo zoo abandon"
        )); // invalid words
    }

    #[test]
    fn zeroize() {
        let mut sut = SUT::sample_other();

        sut.zeroize();

        assert_eq!(sut.words.len(), 0);
    }
}
