import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class BIP39LanguageTests: Test<BIP39Language> {
	func test_wordlist() {
		XCTAssertEqual(SUT.english.wordlist().first?.word, "abandon")
	}
}
