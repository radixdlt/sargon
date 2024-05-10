import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class MnemonicTests: Test<Mnemonic> {
	func test_phrase_roundtrip() throws {
		try eachSample { sut in
			let string = sut.description
			try XCTAssertEqual(SUT(phrase: string), sut)
			try XCTAssertEqual(SUT(phrase: string, language: .english), sut)
			try XCTAssertEqual(SUT(phrase: string, language: nil), sut) // ok to skip language
		}
	}
	
	func test_words_roundtrip() throws {
		try eachSample { sut in
			let words = sut.words
			try XCTAssertEqual(SUT(words: words), sut)
		}
	}
	
	func test_new_from_generated_entropy() throws {
		let wordCounts = BIP39WordCount.allCases
		XCTAssertEqual(wordCounts.count, 5)
		let language = BIP39Language.english
		let n = 100
		func doTest(_ wordCount: Bip39WordCount) throws {
			let mnemonics = try (0..<n).map { _ in
				let sut = SUT(wordCount: wordCount, language: language)
				try XCTAssertEqual(sut, SUT.init(phrase: sut.phrase, language: language))
				return sut
			}
			XCTAssertEqual(Set(mnemonics).count, n)
		}
		try wordCounts.forEach(doTest)
	}
	
	func test_entropy_bytes_throws_wrong_size() {
		XCTAssertThrowsError(try Entropy32Bytes(bytes: Data(repeating: 0xff, count: 36)))
		XCTAssertThrowsError(try Entropy32Bytes(bytes: Data(repeating: 0xff, count: 28)))
		
		XCTAssertThrowsError(try Entropy28Bytes(bytes: Data(repeating: 0xff, count: 32)))
		XCTAssertThrowsError(try Entropy28Bytes(bytes: Data(repeating: 0xff, count: 24)))
		
		XCTAssertThrowsError(try Entropy24Bytes(bytes: Data(repeating: 0xff, count: 28)))
		XCTAssertThrowsError(try Entropy24Bytes(bytes: Data(repeating: 0xff, count: 20)))
		
		XCTAssertThrowsError(try Entropy20Bytes(bytes: Data(repeating: 0xff, count: 24)))
		XCTAssertThrowsError(try Entropy20Bytes(bytes: Data(repeating: 0xff, count: 16)))
		
		XCTAssertThrowsError(try Entropy16Bytes(bytes: Data(repeating: 0xff, count: 20)))
		XCTAssertThrowsError(try Entropy16Bytes(bytes: Data(repeating: 0xff, count: 12)))
	}
}
