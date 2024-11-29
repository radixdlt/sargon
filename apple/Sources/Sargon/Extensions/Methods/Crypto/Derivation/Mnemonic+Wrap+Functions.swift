import Foundation
import SargonUniFFI

extension Mnemonic {
	public var phrase: String {
		mnemonicPhrase(from: self)
	}

	public init(wordCount: BIP39WordCount, language: BIP39Language) {
		let entropy = switch wordCount {
		case .twentyFour:
			BIP39Entropy.entropyOf32Bytes(.generate())
		case .twentyOne:
			BIP39Entropy.entropyOf28Bytes(.generate())
		case .eighteen:
			BIP39Entropy.entropyOf24Bytes(.generate())
		case .fifteen:
			BIP39Entropy.entropyOf20Bytes(.generate())
		case .twelve:
			BIP39Entropy.entropyOf16Bytes(.generate())
		}

		self = newMnemonicGenerateWithEntropy(entropy: entropy, language: language)
	}

	public init(phrase: String, language: BIP39Language? = nil) throws {
		if let language {
			self = try newMnemonicFromPhraseLanguage(phrase: phrase, language: language)
		} else {
			self = try newMnemonicFromPhrase(phrase: phrase)
		}
	}

	public init(words: some Collection<BIP39Word>) throws {
		self = try newMnemonicFromWords(words: Array(words))
	}
}
