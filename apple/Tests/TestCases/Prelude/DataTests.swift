@testable import Sargon

final class RandomBytesTests: XCTestCase {
	func test_hash_of_generated() {
		var set = Set<Data>()
		let n = 100
		(0..<n).forEach { _ in
			set.insert(Data.random(byteCount: 32))
		}
		XCTAssertEqual(set.count, n)
		XCTAssert(Data.random(byteCount: 0).isEmpty)
	}
	
	func test_invalid_hex() throws {
		XCTAssertThrowsError(try Data(hex: "hey"))
		XCTAssertThrowsError(try Data(hex: "abc"))
	}
	
	func test_valid_hex() throws {
		let s = "1234567890abcdef"
		XCTAssertEqual(try Data(hex: s).hex, s)
	}
}
