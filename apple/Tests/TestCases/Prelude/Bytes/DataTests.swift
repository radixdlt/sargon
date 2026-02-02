import CustomDump
import Foundation
@testable import Sargon
import SargonUniFFI
import XCTest

final class RandomBytesTests: XCTestCase {
	func test_hash_of_generated() {
		var set = Set<Data>()
		let n = 100
		for _ in 0 ..< n {
			set.insert(Data.random(byteCount: 32))
		}
		XCTAssertEqual(set.count, n)
		XCTAssert(Data.random(byteCount: 0).isEmpty)
	}

	func test_invalid_hex_non_hex_char() {
		XCTAssertThrowsError(try Data(hex: "nothex"))
		XCTAssertThrowsError(try Data(hex: "abc"))
	}

	func test_invalid_hex_odd_length() {
		XCTAssertThrowsError(try Data(hex: "abc"))
	}

	func test_valid_hex() {
		let s = "1234567890abcdef"
		XCTAssertEqual(try Data(hex: s).hex, s)
	}
}
