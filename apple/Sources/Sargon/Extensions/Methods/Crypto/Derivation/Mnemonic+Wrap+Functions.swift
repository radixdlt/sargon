import SargonUniFFI

extension Mnemonic {
	public var phrase: String {
		mnemonicPhrase(from: self)
	}
	
	public init(phrase: String) throws {
		self = try newMnemonicFromPhrase(phrase: phrase)
	}
	
	public init(phrase: String, language: BIP39Language) throws {
		self = try newMnemonicFromPhraseLanguage(phrase: phrase, language: language)
	}
	
	public init(words: some Collection<BIP39Word>) throws {
		self = try newMnemonicFromWords(words: Array(words))
	}
}
