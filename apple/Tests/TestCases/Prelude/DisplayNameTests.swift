import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

final class DisplayNameTests: Test<DisplayName> {
	func test_too_long_throws() {
		XCTAssertThrowsError(try SUT(validating: "very much too long a name that really does not fit here."))
	}

	func test_codable() throws {
		let raw = "\"Spending Account\"".data(using: .utf8)!
		
		// test decoding
		let sut = try JSONDecoder().decode(SUT.self, from: raw)
		XCTAssertEqual(sut, SUT.sample)
		
		// test encoding
		let encoded = try JSONEncoder().encode(sut)
		try XCTAssertEqual(JSONDecoder().decode(SUT.self, from: encoded), sut)
	}
}
