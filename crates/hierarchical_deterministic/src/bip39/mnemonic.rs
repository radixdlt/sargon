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
    uniffi::Record,
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
impl SafeToLog for Mnemonic {
    /// Logs the word count and FactorSourceID o
    fn non_sensitive(&self) -> impl std::fmt::Debug {
        self.partially_obfuscated_string()
    }
}

impl Mnemonic {
    pub fn to_obfuscated_string(&self) -> String {
        format!("Mnemonic in {} obfuscated.", self.language)
    }

    pub fn from_internal(internal: bip39::Mnemonic) -> Self {
        use k256::elliptic_curve::zeroize::Zeroize;

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

    pub fn to_seed(&self, passphrase: &str) -> BIP39Seed {
        // BIP39Seed::new(self.internal().to_seed(passphrase))
        BIP39Seed(Exactly64Bytes::from(&self.internal().to_seed(passphrase)))
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
    fn non_sensitive() {
        let mnemonic = SUT::sample();
        assert_eq!(
            format!("{:?}", mnemonic.non_sensitive()),
            format!("{:?}", "24 words (bright...mandate)")
        );
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
