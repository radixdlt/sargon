import SargonUniFFI
import Foundation

public typealias BIP39Entropy = Bip39Entropy

extension Entropy16Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy16BytesFromBytes(bytes: Data(bytes))
	}
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: 16))
	}
}

extension Entropy20Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy20BytesFromBytes(bytes: Data(bytes))
	}
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: 20))
	}
}

extension Entropy24Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy24BytesFromBytes(bytes: Data(bytes))
	}
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: 24))
	}
}

extension Entropy28Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy28BytesFromBytes(bytes: Data(bytes))
	}
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: 28))
	}
}

extension Entropy32Bytes {
	public init(bytes: some DataProtocol) throws {
		self = try newEntropy32BytesFromBytes(bytes: Data(bytes))
	}
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: 32))
	}
}

extension Mnemonic {
	public var phrase: String {
		mnemonicPhrase(from: self)
	}
	
	public init(wordCount: BIP39WordCount, language: BIP39Language) {
		let entropy: BIP39Entropy = switch wordCount {
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
