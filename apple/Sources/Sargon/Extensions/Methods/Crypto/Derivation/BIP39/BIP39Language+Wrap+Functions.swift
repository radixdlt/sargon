import Foundation
import SargonUniFFI

extension BIP39Language {
	public func wordlist() -> [BIP39Word] {
		bip39LanguageWordlist(language: self)
	}
}
