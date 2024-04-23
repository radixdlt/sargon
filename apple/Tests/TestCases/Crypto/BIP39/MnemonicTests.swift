import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class MnemonicTests: Test<Mnemonic> {
	func test_phrase_roundtrip() throws {
		func doTest(_ sut: SUT) throws {
			let string = sut.description
			try XCTAssertEqual(SUT(phrase: string), sut)
			try XCTAssertEqual(SUT(phrase: string, language: .english), sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
	
	func test_words_roundtrip() throws {
		func doTest(_ sut: SUT) throws {
			let words = sut.words
			try XCTAssertEqual(SUT(words: words), sut)
		}
		try SUT.sampleValues.forEach(doTest)
	}
}
